use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{model::Seeds, SceneState};
use crate::plugin::loader_plugin::{LevelLoadingUpdated, LoadLevels, LoaderPlugin};

#[derive(Clone)]
pub struct HardReset();

pub fn setup(appx: &mut App) {
    appx.add_plugin(LoaderPlugin)
        .add_enter_system(SceneState::Loading, loading_enter)
        .add_event::<HardReset>()
        .add_system(
            loading_updated
                .run_in_state(SceneState::Loading)
                .run_on_event::<LevelLoadingUpdated>(),
        )
        .add_system(
            hard_reset
                .run_not_in_state(SceneState::Loading)
                .run_on_event::<HardReset>(),
        );
}

fn loading_enter(mut commands: Commands) {
    commands.insert_resource(LoadLevels::new(r"level/index.toml"));
}

fn loading_updated(
    mut commands: Commands,
    mut events: EventReader<LevelLoadingUpdated>,
    mut progress: Local<(usize, usize)>,
) {
    for event in events.iter() {
        use LevelLoadingUpdated::*;
        match event {
            Loading { total, done } => {
                if progress.0 != *total || progress.1 != *done {
                    *progress = (*total, *done);
                    info!("Loading: {}/{}", done, total);
                }
            }
            Failure { which } => error!("Failed to load: {}", which),
            Success { seeds } => {
                commands.insert_resource(Seeds::from(seeds.clone()));
                commands.insert_resource(NextState(SceneState::Running));
            }
        }
    }
}

fn hard_reset(mut commands: Commands, mut events: EventReader<HardReset>) {
    for _ in events.iter().last() {
        commands.insert_resource(NextState(SceneState::Loading));
        commands.insert_resource(LoadLevels::new(r"level/index.toml"));
    }
}
