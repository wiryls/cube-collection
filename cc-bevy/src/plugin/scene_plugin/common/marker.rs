use bevy::prelude::*;

/// Earthbound marks that the current object belongs to a level. These objects
/// will be removed when we switch levels.
#[derive(Component, Default)]
pub struct Earthbound;
