use super::level;
use serde::Deserialize;
use snafu::{ensure, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)), context(suffix(false)))]
pub enum Error {
    #[snafu(display("missing field '{}'", name))]
    MissingField { name: &'static str },

    #[snafu(display("expect map marker string, but get '{}'", what))]
    InvalidMarker { what: String },

    #[snafu(display("expect copiable element at ({}, {})", pos.0, pos.1))]
    Uncopiable { pos: (i32, i32) },

    #[snafu(display("expect mergeable elements at ({}, {}) and ({}, {})", lhs.0, lhs.1, rhs.0, rhs.1))]
    Unmergeable { lhs: (i32, i32), rhs: (i32, i32) },

    #[snafu(display("expect movement string, but get '{}'", what))]
    InvalidMovement { what: char },

    #[snafu(display("expect a valid location, but get ({}, {})", pos.0, pos.1))]
    InvalidLocation { pos: (i32, i32) },
}

#[derive(Debug, Deserialize)]
pub struct Source {
    info: Info,
    map: Map,
}

#[derive(Debug, Deserialize)]
struct Info {
    title: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct Map {
    raw: String,
    movements: Vec<Movement>,
}

#[derive(Debug, Deserialize)]
struct Movement {
    command: String,
    is_loop: bool,
    binding: Vec<[i32; 2]>,
}

impl Source {
    pub fn into_level(self) -> Result<level::Level, Error> {
        ensure!(
            !self.info.title.is_empty(),
            MissingField { name: "info.title" }
        );
        ensure!(
            !self.info.author.is_empty(),
            MissingField {
                name: "info.author"
            }
        );
        ensure!(!self.map.raw.is_empty(), MissingField { name: "map.raw" });

        let mut builder: LevelBuilder = self.info.into();
        for line in self.map.raw.lines() {
            for c in line.chars() {
                match c {
                    'W' => builder.make_cube(level::CubeType::White),
                    'R' => builder.make_cube(level::CubeType::Red),
                    'B' => builder.make_cube(level::CubeType::Blue),
                    'G' => builder.make_cube(level::CubeType::Green),
                    'x' => builder.make_destination(),
                    ' ' => builder.make_empty(),
                    '~' => builder.copy_left()?,
                    '|' => builder.copy_upper()?,
                    '/' => builder.copy_upper_and_left()?,
                    _ => ensure!(false, InvalidMarker { what: c.to_owned() }),
                }
            }
            builder.mark_line_end();
        }

        fn add<'a>(builder: &'a mut CommandBuider, string: &mut String) -> &'a mut CommandBuider {
            if !string.is_empty() {
                if let Ok(i) = string.parse::<i32>() {
                    builder.add(i);
                    string.clear();
                }
            }
            builder
        }
        for m in self.map.movements {
            let mut n = String::new();
            let mut b = CommandBuider::new(m.is_loop);
            for c in m.command.chars() {
                match c {
                    'I' => add(&mut b, &mut n).put(level::Movement::Idle),
                    'L' => add(&mut b, &mut n).put(level::Movement::Left),
                    'D' => add(&mut b, &mut n).put(level::Movement::Down),
                    'U' => add(&mut b, &mut n).put(level::Movement::Up),
                    'R' => add(&mut b, &mut n).put(level::Movement::Right),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if !b.is_empty() => {
                        n.push(c)
                    }
                    _ => ensure!(false, InvalidMovement { what: c }),
                }
            }

            let c: level::Command = b.into();
            for p in m.binding {
                builder.bind_command(p[0], p[1], c.clone())?;
            }
        }

        Ok(builder.into())
    }
}

#[derive(Debug, Default)]
struct LevelBuilder {
    // output
    i: level::Info,
    h: i32,
    w: i32,
    cs: Vec<level::Cube>,
    ds: Vec<level::Location>,

    // cached
    x: i32,
    m: Indexer,
}

impl Into<LevelBuilder> for Info {
    fn into(self) -> LevelBuilder {
        LevelBuilder::new(level::Info {
            title: self.title,
            author: self.author,
        })
    }
}

impl Into<level::Level> for LevelBuilder {
    fn into(mut self) -> level::Level {
        self.cs.retain(|c| !c.body.is_empty());
        level::Level {
            info: self.i,
            size: level::Size {
                width: self.w,
                height: self.h,
            },
            cube: self.cs,
            dest: self.ds,
        }
    }
}

impl LevelBuilder {
    fn new(i: level::Info) -> Self {
        Self {
            i: level::Info {
                title: i.title,
                author: i.author,
            },
            ..Default::default()
        }
    }

    fn mark_line_end(&mut self) {
        self.h += 1;
        self.w = self.w.max(self.x);
        self.x = 0;
        self.m.make_row();
    }

    fn make_empty(&mut self) {
        self.x += 1;
        self.m.put(None);
    }

    fn make_destination(&mut self) {
        let l = level::Location {
            x: self.x,
            y: self.h,
        };

        self.x += 1;
        self.m.put(None);
        self.ds.push(l);
    }

    fn make_cube(&mut self, kind: level::CubeType) {
        let i = self.cs.len();
        let c = level::Cube {
            kind,
            body: vec![level::Location {
                x: self.x,
                y: self.h,
            }],
            ..Default::default()
        };

        self.x += 1;
        self.m.put(Some(i));
        self.cs.push(c);
    }

    fn copy_left(&mut self) -> Result<(), Error> {
        let x = self.x - 1;
        let y = self.h;
        match self
            .m
            .get(x, y)
            .and_then(|i| self.cs.get_mut(i).map(|c| (i, c)))
        {
            None => Err(Error::Uncopiable { pos: (x, y) }),
            Some((i, c)) => {
                self.x += 1;
                self.m.put(Some(i));
                c.body.push(level::Location { x: x + 1, y });
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
            None => Err(Error::Uncopiable { pos: (x, y) }),
            Some((i, c)) => {
                self.x += 1;
                self.m.put(Some(i));
                c.body.push(level::Location { x, y: y + 1 });
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
                    c.body.push(level::Location {
                        x: upper.0,
                        y: left.1,
                    });
                }

                // as usual
                self.x += 1;
                self.m.put(Some(l));

                true
            }
            _ => false,
        };

        ensure!(
            ok,
            Unmergeable {
                lhs: upper,
                rhs: left
            }
        );
        Ok(())
    }

    fn bind_command(&mut self, x: i32, y: i32, command: level::Command) -> Result<(), Error> {
        match self.m.get(x, y).and_then(|i| self.cs.get_mut(i)) {
            Some(x) => Ok(x.command = Some(command)),
            None => Err(Error::InvalidLocation { pos: (x, y) }),
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
struct CommandBuider(level::Command);

impl Into<level::Command> for CommandBuider {
    fn into(mut self) -> level::Command {
        self.0.movements.retain(|m| m.0 > 0);
        self.0
    }
}

impl CommandBuider {
    fn new(is_loop: bool) -> Self {
        Self(level::Command {
            is_loop,
            movements: Vec::new(),
        })
    }

    fn put(&mut self, movement: level::Movement) {
        match self.0.movements.last_mut() {
            Some(c) if c.1 == movement => c.0 += 1,
            _ => self.0.movements.push((1, movement)),
        }
    }

    fn add(&mut self, number: i32) {
        match self.0.movements.last_mut() {
            Some(c) => c.0 += number - 1,
            _ => self.0.movements.push((number, level::Movement::Idle)),
        }
    }

    fn is_empty(&self) -> bool {
        self.0.movements.is_empty()
    }
}
