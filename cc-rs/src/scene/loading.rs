use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::state::State;
use crate::extra::load;
use crate::model::seed;

/// - input: none
/// - output: ```Res<seed::Seeds>```
pub struct LoadingScene;
impl Plugin for LoadingScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(load::LoaderPlugin)
            .add_enter_system(State::Loading, loading_enter)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(State::Loading)
                    .with_system(loading_status)
                    .with_system(loading_success.run_if_resource_exists::<seed::Seeds>())
                    .into(),
            );
    }
}

fn loading_enter(mut commands: Commands) {
    commands.insert_resource(load::LoadSeeds::new(r"level/index.toml"));
}

fn loading_status(mut status: EventReader<load::LoadSeedsUpdated>) {
    for event in status.iter() {
        use load::LoadSeedsUpdated::{Failure, Loading};
        match event {
            Loading { total, done } => println!("Loading: {}/{}", done, total),
            Failure { which } => println!("Loading: {}", which),
        }
    }
}

fn loading_success(mut commands: Commands) {
    commands.insert_resource(NextState(State::Running));
}
