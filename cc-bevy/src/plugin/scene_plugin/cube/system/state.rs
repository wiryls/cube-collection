use bevy::prelude::*;
use cc_core::cube::Movement;
use std::collections::VecDeque;

use super::{
    super::{
        super::input::MovementChanged,
        component::Cubic,
        world::{Input, World},
    },
    translate::Translate,
};

pub fn system(
    mut commands: Commands,
    mut input: EventReader<MovementChanged>,
    mut cubes: Query<(Entity, &mut Cubic)>,
    mut world: ResMut<World>,
    mut actions: Local<ActionQueue>,
    time: Res<Time>,
) {
    // Update actions
    for action in input.iter() {
        match action {
            MovementChanged::Add(m) => actions.add(*m),
            MovementChanged::Set(m) => actions.set(*m),
        };
    }

    // Update world.
    let diffs = world.next(time.delta(), &mut *actions);
    if !diffs.is_empty() {
        for (id, mut cube, diff) in cubes
            .iter_mut()
            .filter_map(|(id, cube)| diffs.get(&cube.id).map(|diff| (id, cube, diff)))
        {
            // color
            if let Some(value) = diff.kind {
                // TODO: add another component
                cube.kind = value;
            }

            // shape
            if let Some(value) = diff.neighborhood {
                // TODO: add another component
                cube.neighborhood = value;
            }

            // translation
            if let Some(component) = Translate::make(&*cube, diff) {
                commands.entity(id).insert(component);
            }
            if let Some(value) = diff.position {
                cube.position = value;
            }
            if let Some(value) = diff.movement {
                cube.movement = value;
            }
            if let Some(value) = diff.constraint {
                cube.constraint = value;
            }
        }
    }
}

#[derive(Default)]
pub struct ActionQueue {
    once: VecDeque<Movement>,
    repeat: Option<Movement>,
}

impl ActionQueue {
    fn add(&mut self, movement: Movement) {
        if let Some(movement) = self.repeat.replace(movement) {
            self.once.push_back(movement);
        }
    }

    fn set(&mut self, movement: Option<Movement>) {
        match movement {
            None => self.repeat = None,
            Some(movement) => {
                self.once.clear();
                self.once.push_back(movement);
                self.repeat = Some(movement);
            }
        }
    }

    fn pop(&mut self) -> Option<Movement> {
        let output = match self.once.pop_front() {
            None => self.repeat,
            Some(movement) => Some(movement),
        };
        output
    }
}

impl Input for ActionQueue {
    fn fetch(&mut self) -> Option<Movement> {
        self.pop()
    }
}
