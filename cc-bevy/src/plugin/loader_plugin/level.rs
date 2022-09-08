use cc_core::{cube, seed};
use serde::Deserialize;
use snafu::{ensure, Snafu};

use super::LevelSeed;

/////////////////////////////////////////////////////////////////////////////
// Source and Error

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum LevelError {
    #[snafu(display("missing field '{}'", field))]
    MissingField { field: &'static str },

    #[snafu(display("expect map marker string, but get '{}'", character))]
    InvalidMarker { character: char },

    #[snafu(display("expect copiable element at ({}, {})", position.0, position.1))]
    Uncopiable { position: (i32, i32) },

    #[snafu(display("expect mergeable elements at ({}, {}) and ({}, {})", this.0, this.1, that.0, that.1))]
    Unmergeable { this: (i32, i32), that: (i32, i32) },

    #[snafu(display("expect movement string, but get '{}'", character))]
    InvalidMovement { character: char },

    #[snafu(display("expect a valid location, but get ({}, {})", position.0, position.1))]
    InvalidLocation { position: (i32, i32) },
}

#[derive(Deserialize)]
pub struct LevelSource {
    info: Info,
    map: Map,
}

#[derive(Deserialize)]
struct Info {
    title: String,
    author: String,
}

#[derive(Deserialize)]
struct Map {
    raw: String,
    commands: Option<Vec<Command>>,
}

#[derive(Deserialize)]
struct Command {
    content: String,
    looping: bool,
    binding: Vec<[i32; 2]>,
}

impl LevelSource {
    pub fn into_seed(self) -> Result<LevelSeed, LevelError> {
        ensure!(
            !self.info.title.is_empty(),
            MissingField {
                field: "info.title"
            }
        );
        ensure!(
            !self.info.author.is_empty(),
            MissingField {
                field: "info.author"
            }
        );
        ensure!(!self.map.raw.is_empty(), MissingField { field: "map.raw" });

        let mut parser: LevelParser = self.info.into();
        for line in self.map.raw.lines() {
            for c in line.chars() {
                match c {
                    'W' => parser.make_cube(cube::Kind::White),
                    'R' => parser.make_cube(cube::Kind::Red),
                    'B' => parser.make_cube(cube::Kind::Blue),
                    'G' => parser.make_cube(cube::Kind::Green),
                    'x' => parser.make_destination(),
                    ' ' => parser.make_empty(),
                    '~' => parser.copy_left()?,
                    '|' => parser.copy_upper()?,
                    '/' => parser.copy_upper_and_left()?,
                    _ => ensure!(false, InvalidMarker { character: c }),
                }
            }
            parser.mark_line_end();
        }

        for m in self.map.commands.unwrap_or_default() {
            let mut n = String::new();
            let mut p = CommandParser::new(m.looping);
            for c in m.content.chars() {
                match c {
                    'I' => put(&mut p, &mut n).put(None),
                    'L' => put(&mut p, &mut n).put(Some(cube::Movement::Left)),
                    'D' => put(&mut p, &mut n).put(Some(cube::Movement::Down)),
                    'U' => put(&mut p, &mut n).put(Some(cube::Movement::Up)),
                    'R' => put(&mut p, &mut n).put(Some(cube::Movement::Right)),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if !p.is_empty() => {
                        n.push(c)
                    }
                    _ => ensure!(false, InvalidMovement { character: c }),
                }
            }

            let c: seed::Command = p.into();
            for p in m.binding {
                parser.bind_command(p[0], p[1], c.clone())?;
            }
        }
        fn put<'a>(parser: &'a mut CommandParser, buffer: &mut String) -> &'a mut CommandParser {
            if !buffer.is_empty() {
                if let Ok(i) = buffer.parse::<i32>() {
                    parser.add(i);
                    buffer.clear();
                }
            }
            parser
        }

        Ok(LevelSeed::new(parser.into()))
    }
}

/////////////////////////////////////////////////////////////////////////////
// Parsers

struct LevelParser {
    // output
    i: seed::Info,
    h: i32,
    w: i32,
    cs: Vec<seed::Cube>,
    ds: Vec<cube::Point>,

    // cached
    x: i32,
    m: LevelMapBuilder,
}

impl Into<LevelParser> for Info {
    fn into(self) -> LevelParser {
        let (title, author) = (self.title, self.author);
        LevelParser::new(seed::Info { title, author })
    }
}

impl Into<seed::Seed> for LevelParser {
    fn into(mut self) -> seed::Seed {
        self.cs.retain(|c| !c.body.is_empty());
        seed::Seed {
            info: self.i,
            size: seed::Size {
                width: self.w,
                height: self.h,
            },
            cubes: self.cs,
            destnations: self.ds,
        }
    }
}

impl LevelParser {
    fn new(i: seed::Info) -> Self {
        let (title, author) = (i.title, i.author);
        Self {
            i: seed::Info { title, author },
            h: 0,
            w: 0,
            cs: Vec::new(),
            ds: Vec::new(),
            x: 0,
            m: LevelMapBuilder(Vec::new()),
        }
    }

    fn make(&mut self, value: Option<usize>) {
        self.x += 1;
        self.m.put(value);
    }

    fn mark_line_end(&mut self) {
        self.h += 1;
        self.w = self.w.max(self.x);
        self.x = 0;
        self.m.make_row();
    }

    fn make_empty(&mut self) {
        self.make(None);
    }

    fn make_destination(&mut self) {
        self.ds.push(cube::Point::new(self.x, self.h));
        self.make(None);
    }

    fn make_cube(&mut self, kind: cube::Kind) {
        let i = self.cs.len();
        let c = seed::Cube {
            kind,
            body: vec![cube::Point::new(self.x, self.h)],
            command: None,
        };

        self.cs.push(c);
        self.make(Some(i));
    }

    fn copy_left(&mut self) -> Result<(), LevelError> {
        let x = self.x - 1;
        let y = self.h;
        match self
            .m
            .get(x, y)
            .and_then(|i| self.cs.get_mut(i).map(|c| (i, c)))
        {
            None => Err(LevelError::Uncopiable { position: (x, y) }),
            Some((i, c)) => {
                c.body.push(cube::Point::new(x + 1, y));
                self.make(Some(i));
                Ok(())
            }
        }
    }

    fn copy_upper(&mut self) -> Result<(), LevelError> {
        let x = self.x;
        let y = self.h - 1;
        match self
            .m
            .get(x, y)
            .and_then(|i| self.cs.get_mut(i).map(|c| (i, c)))
        {
            None => Err(LevelError::Uncopiable { position: (x, y) }),
            Some((i, c)) => {
                c.body.push(cube::Point::new(x, y + 1));
                self.make(Some(i));
                Ok(())
            }
        }
    }

    fn copy_upper_and_left(&mut self) -> Result<(), LevelError> {
        let upper = (self.x, self.h - 1);
        let left = (self.x - 1, self.h);

        let lhs = self
            .m
            .get(upper.0, upper.1)
            .and_then(|i| self.cs.get(i).map(|c| (i, c)));
        let rhs = self
            .m
            .get(left.0, left.1)
            .and_then(|i| self.cs.get(i).map(|c| (i, c)));

        let ok = match (lhs, rhs) {
            (Some(l), Some(r)) if l.0 == r.0 => true,
            (Some(l), Some(r)) if l.1.kind != r.1.kind => false,
            (Some(l), Some(r)) => {
                // the lower index, the higher priority
                let (l, r) = if l.0 < r.0 { (l.0, r.0) } else { (r.0, l.0) };

                // move r into l
                let mut v = Vec::new();
                if let Some(c) = self.cs.get_mut(r) {
                    std::mem::swap::<Vec<_>>(v.as_mut(), c.body.as_mut());
                }
                for o in v.iter() {
                    if let Some(i) = self.m.get_mut(o.x, o.y) {
                        *i = l;
                    }
                }
                if let Some(c) = self.cs.get_mut(l) {
                    c.body.append(v.as_mut());
                    c.body.push(cube::Point::new(upper.0, left.1));
                }

                // as usual
                self.make(Some(l));
                true
            }
            _ => false,
        };

        if ok {
            Ok(())
        } else {
            let (this, that) = (upper, left);
            Err(LevelError::Unmergeable { this, that })
        }
    }

    fn bind_command(&mut self, x: i32, y: i32, command: seed::Command) -> Result<(), LevelError> {
        match self.m.get(x, y).and_then(|i| self.cs.get_mut(i)) {
            Some(x) => Ok(x.command = Some(command)),
            None => Err(LevelError::InvalidLocation { position: (x, y) }),
        }
    }
}

struct LevelMapBuilder(
    Vec<Vec<Option<usize>>>, /* add a DisjointSet if needed */
);

impl LevelMapBuilder {
    fn make_row(&mut self) {
        self.0.push(Vec::new());
    }

    fn make_row_with(&mut self, value: Option<usize>) {
        self.0.push(vec![value]);
    }

    fn put(&mut self, value: Option<usize>) {
        match self.0.last_mut() {
            None => self.make_row_with(value),
            Some(v) => v.push(value),
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<usize> {
        match self.0.get(y as usize) {
            None => None,
            Some(v) => match v.get(x as usize) {
                None => None,
                Some(i) => i.to_owned(),
            },
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut usize> {
        match self.0.get_mut(y as usize) {
            None => None,
            Some(v) => match v.get_mut(x as usize) {
                None => None,
                Some(i) => i.as_mut(),
            },
        }
    }
}

struct CommandParser(seed::Command);

impl Into<seed::Command> for CommandParser {
    fn into(mut self) -> seed::Command {
        self.0.movements.retain(|m| m.1 > 0);
        self.0
    }
}

impl CommandParser {
    fn new(is_loop: bool) -> Self {
        Self(seed::Command {
            is_loop,
            movements: Vec::new(),
        })
    }

    fn put(&mut self, movement: Option<cube::Movement>) {
        match self.0.movements.last_mut() {
            Some(c) if c.0 == movement => c.1 += 1,
            _ => self.0.movements.push((movement, 1)),
        }
    }

    fn add(&mut self, number: i32) {
        match self.0.movements.last_mut() {
            Some(c) => c.1 = c.1 + number as usize - 1,
            _ => self.0.movements.push((None, number as usize)),
        }
    }

    fn is_empty(&self) -> bool {
        self.0.movements.is_empty()
    }
}
