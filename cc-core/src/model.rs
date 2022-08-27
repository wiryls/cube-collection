mod item;
mod kind;
mod motion;
mod movement;
mod neighborhood;
mod point;

pub(crate) use motion::{Agreement, Motion};
#[allow(unused_imports)]
pub(crate) use neighborhood::Adjacence;

pub use item::{Diff, Item};
pub use kind::Kind;
pub use movement::{Constraint, Movement};
pub use neighborhood::Neighborhood;
pub use point::Point;
