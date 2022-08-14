use crate::model::seed::CubeWorldSeed;
use cc_core::{model, seed};
use serde::Deserialize;
use snafu::{ensure, Snafu};

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
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
pub struct Source {
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
    is_loop: bool,
    binding: Vec<[i32; 2]>,
}

impl Source {
    pub fn into_seed(self) -> Result<CubeWorldSeed, Error> {
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

        let mut builder: LevelBuilder = self.info.into();
        for line in self.map.raw.lines() {
            for c in line.chars() {
                match c {
                    'W' => builder.make_cube(model::Kind::White),
                    'R' => builder.make_cube(model::Kind::Red),
                    'B' => builder.make_cube(model::Kind::Blue),
                    'G' => builder.make_cube(model::Kind::Green),
                    'x' => builder.make_destination(),
                    ' ' => builder.make_empty(),
                    '~' => builder.copy_left()?,
                    '|' => builder.copy_upper()?,
                    '/' => builder.copy_upper_and_left()?,
                    _ => ensure!(false, InvalidMarker { character: c }),
                }
            }
            builder.mark_line_end();
        }

        for m in self.map.commands.unwrap_or_default() {
            let mut n = String::new();
            let mut b = CommandBuilder::new(m.is_loop);
            for c in m.content.chars() {
                match c {
                    'I' => put(&mut b, &mut n).put(None),
                    'L' => put(&mut b, &mut n).put(Some(model::Movement::Left)),
                    'D' => put(&mut b, &mut n).put(Some(model::Movement::Down)),
                    'U' => put(&mut b, &mut n).put(Some(model::Movement::Up)),
                    'R' => put(&mut b, &mut n).put(Some(model::Movement::Right)),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if !b.is_empty() => {
                        n.push(c)
                    }
                    _ => ensure!(false, InvalidMovement { character: c }),
                }
            }

            let c: seed::Command = b.into();
            for p in m.binding {
                builder.bind_command(p[0], p[1], c.clone())?;
            }
        }
        fn put<'a>(builder: &'a mut CommandBuilder, buffer: &mut String) -> &'a mut CommandBuilder {
            if !buffer.is_empty() {
                if let Ok(i) = buffer.parse::<i32>() {
                    builder.add(i);
                    buffer.clear();
                }
            }
            builder
        }

        Ok(CubeWorldSeed::new(builder.into()))
    }
}

struct LevelBuilder {
    // output
    i: seed::Info,
    h: i32,
    w: i32,
    cs: Vec<seed::Cube>,
    ds: Vec<model::Point>,

    // cached
    x: i32,
    m: Indexer,
}

impl Into<LevelBuilder> for Info {
    fn into(self) -> LevelBuilder {
        LevelBuilder::new(seed::Info {
            title: self.title,
            author: self.author,
        })
    }
}

impl Into<seed::Seed> for LevelBuilder {
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

impl LevelBuilder {
    fn new(i: seed::Info) -> Self {
        Self {
            i: seed::Info {
                title: i.title,
                author: i.author,
            },
            h: 0,
            w: 0,
            cs: Vec::new(),
            ds: Vec::new(),
            x: 0,
            m: Indexer(Vec::new()),
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
        self.ds.push(model::Point::new(self.x, self.h));
        self.make(None);
    }

    fn make_cube(&mut self, kind: model::Kind) {
        let i = self.cs.len();
        let c = seed::Cube {
            kind,
            body: vec![model::Point::new(self.x, self.h)],
            command: None,
        };

        self.cs.push(c);
        self.make(Some(i));
    }

    fn copy_left(&mut self) -> Result<(), Error> {
        let x = self.x - 1;
        let y = self.h;
        match self
            .m
            .get(x, y)
            .and_then(|i| self.cs.get_mut(i).map(|c| (i, c)))
        {
            None => Err(Error::Uncopiable { position: (x, y) }),
            Some((i, c)) => {
                c.body.push(model::Point::new(x + 1, y));
                self.make(Some(i));
                Ok(())
            }
        }
    }

    fn copy_upper(&mut self) -> Result<(), Error> {
        let x = self.x;
        let y = self.h - 1;
        match self
            .m
            .get(x, y)
            .and_then(|i| self.cs.get_mut(i).map(|c| (i, c)))
        {
            None => Err(Error::Uncopiable { position: (x, y) }),
            Some((i, c)) => {
                c.body.push(model::Point::new(x, y + 1));
                self.make(Some(i));
                Ok(())
            }
        }
    }

    fn copy_upper_and_left(&mut self) -> Result<(), Error> {
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
                    c.body.push(model::Point::new(upper.0, left.1));
                }

                // as usual
                self.make(Some(l));
                true
            }
            _ => false,
        };

        ensure!(
            ok,
            Unmergeable {
                this: upper,
                that: left
            }
        );
        Ok(())
    }

    fn bind_command(&mut self, x: i32, y: i32, command: seed::Command) -> Result<(), Error> {
        match self.m.get(x, y).and_then(|i| self.cs.get_mut(i)) {
            Some(x) => Ok(x.command = Some(command)),
            None => Err(Error::InvalidLocation { position: (x, y) }),
        }
    }
}

struct Indexer(Vec<Vec<Option<usize>>>);

impl Indexer {
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

struct CommandBuilder(seed::Command);

impl Into<seed::Command> for CommandBuilder {
    fn into(mut self) -> seed::Command {
        self.0.movements.retain(|m| m.1 > 0);
        self.0
    }
}

impl CommandBuilder {
    fn new(is_loop: bool) -> Self {
        Self(seed::Command {
            is_loop,
            movements: Vec::new(),
        })
    }

    fn put(&mut self, movement: Option<model::Movement>) {
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
