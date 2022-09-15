use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{model::Seeds, SceneState};
use crate::plugin::loader_plugin::{LevelLoadingUpdated, LoadLevels, LoaderPlugin};

pub fn setup(appx: &mut App) {
    appx.add_plugin(LoaderPlugin)
        .add_enter_system(SceneState::Loading, loading_enter)
        .add_system(loading_updated.run_in_state(SceneState::Loading));
}

fn loading_enter(mut commands: Commands) {
    commands.insert_resource(LoadLevels::new(r"level/index.toml"));
}

fn loading_updated(mut commands: Commands, mut events: EventReader<LevelLoadingUpdated>) {
    for event in events.iter() {
        use LevelLoadingUpdated::*;
        match event {
            Loading { total, done } => info!("Loading: {}/{}", done, total),
            Failure { which } => error!("Failed to load: {}", which),
            Success { seeds } => {
                commands.insert_resource(Seeds::from(seeds.clone()));
                commands.insert_resource(NextState(SceneState::Running));
            }
        }
    }
}