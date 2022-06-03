use super::location::Location;
use std::collections::HashMap;

#[derive(Default)]
pub struct Lookup(HashMap<u64, usize>);

#[allow(dead_code)]
impl Lookup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from<'a, I, T, U>(it: I) -> Self
    where
        I: Iterator<Item = &'a T>,
        T: 'a + Location<U> + ?Sized,
        U: Into<i32> + ?Sized,
    {
        Self(it.enumerate().map(|(i, o)| (Self::key(o), i)).collect())
    }

    pub fn put<T, U>(&mut self, o: &T) -> &mut Self
    where
        T: Location<U> + ?Sized,
        U: Into<i32> + ?Sized,
    {
        self.0.insert(Self::key(o), self.0.len());
        self
    }

    pub fn get<T, U>(&self, o: &T) -> Option<usize>
    where
        T: Location<U> + ?Sized,
        U: Into<i32> + ?Sized,
    {
        self.0.get(&Self::key(o)).map(|x| x.to_owned())
    }

    fn key<T, U>(o: &T) -> u64
    where
        T: Location<U> + ?Sized,
        U: Into<i32> + ?Sized,
    {
        ((o.x().into() as u64) << 32) | (o.y().into() as u64)
    }
}
