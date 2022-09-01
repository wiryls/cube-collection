use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use cc_core::cube::Movement;

use super::Lable;

pub fn setup(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
            .label(Lable::INPUT)
            .after(Lable::VIEW)
            .with_system(keyboard)
            .into(),
    );
}

fn keyboard(mut input: EventReader<KeyboardInput>, mut actions: Local<ActionInput>) {
    // try to calculate a command and send it to movement system.
    for (code, key) in input
        .iter()
        .filter_map(|key| key.key_code.map(|code| (code, key)))
    {
        enum Then {
            Movement(Option<Movement>),
            Nothing,
        }

        let presse = key.state.is_pressed();
        let output = match code {
            KeyCode::W | KeyCode::Up => Then::Movement(actions.update(Movement::Up, presse)),
            KeyCode::A | KeyCode::Left => Then::Movement(actions.update(Movement::Left, presse)),
            KeyCode::S | KeyCode::Down => Then::Movement(actions.update(Movement::Down, presse)),
            KeyCode::R | KeyCode::Right => Then::Movement(actions.update(Movement::Right, presse)),
            _ => Then::Nothing,
        };

        match output {
            Then::Movement(movement) => todo!(),
            Then::Nothing => todo!(),
        }
    }

    // todo!()
}

#[derive(Default /* required by Local */)]
struct ActionInput(Vec<Movement>);

impl ActionInput {
    fn update(&mut self, movement: Movement, pressed: bool) -> Option<Movement> {
        // self.0.retain(||)
        None
    }
}
