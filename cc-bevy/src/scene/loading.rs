use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::game_state::GameState;
use crate::model::seed::CubeWorldSeeds;
use crate::plugin::load::{LoadSeeds, LoadSeedsUpdated, LoaderPlugin};

/// - input: none
/// - output: ```Res<CubeWorldSeeds>```
pub struct LoadingScene;
impl Plugin for LoadingScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoaderPlugin)
            .add_enter_system(GameState::Loading, loading_enter)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Loading)
                    .with_system(loading_status)
                    .with_system(loading_success.run_if_resource_exists::<CubeWorldSeeds>())
                    .into(),
            );
    }
}

fn loading_enter(mut commands: Commands) {
    commands.insert_resource(LoadSeeds::new(r"level/index.toml"));
}

fn loading_status(mut status: EventReader<LoadSeedsUpdated>) {
    for event in status.iter() {
        use LoadSeedsUpdated::{Failure, Loading};
        match event {
            Loading { total, done } => println!("Loading: {}/{}", done, total),
            Failure { which } => println!("Loading: {}", which),
        }
    }
}

fn loading_success(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Running));
}
