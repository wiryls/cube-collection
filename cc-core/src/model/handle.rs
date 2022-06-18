#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct HeadID(usize);

impl From<usize> for HeadID {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
impl From<HeadID> for usize {
    fn from(i: HeadID) -> Self {
        i.0
    }
}
impl From<&HeadID> for usize {
    fn from(i: &HeadID) -> Self {
        i.0
    }
}
impl From<&mut HeadID> for usize {
    fn from(i: &mut HeadID) -> Self {
        i.0
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UnitID(usize);

impl From<usize> for UnitID {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
impl From<UnitID> for usize {
    fn from(i: UnitID) -> Self {
        i.0
    }
}
impl From<&UnitID> for usize {
    fn from(i: &UnitID) -> Self {
        i.0
    }
}
impl From<&mut UnitID> for usize {
    fn from(i: &mut UnitID) -> Self {
        i.0
    }
}
