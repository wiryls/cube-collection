use super::{rule::world::Command, Lable};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn setup(app: &mut App) {
    app.add_system(keyboard.label(Lable::INPUT).after(Lable::VIEW));
}

fn keyboard(mut commands: Local<Vec<Command>>, mut input: EventReader<KeyboardInput>) {
    // try to calculate a command and send it to movement system.
    // todo!()
}
