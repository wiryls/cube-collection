use super::location::Location;
use std::collections::HashMap;

#[derive(Default)]
pub struct Lookup(HashMap<u64, usize>);

#[allow(dead_code)]
impl Lookup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from<'a, I, U, V>(it: I) -> Self
    where
        I: Iterator<Item = &'a U>,
        U: 'a + Location<V>,
        V: Into<i32>,
    {
        Self(it.enumerate().map(|(i, o)| (Self::key(o), i)).collect())
    }

    pub fn put<U: Location<V>, V: Into<i32>>(&mut self, o: &U) -> &mut Self {
        self.0.insert(Self::key(o), self.0.len());
        self
    }

    pub fn get<U: Location<V>, V: Into<i32>>(&self, o: &U) -> Option<usize> {
        self.0.get(&Self::key(o)).map(|x| x.to_owned())
    }

    fn key<U: Location<V>, V: Into<i32>>(o: &U) -> u64 {
        ((o.x_().into() as u64) << 32) | (o.y_().into() as u64)
    }
}
