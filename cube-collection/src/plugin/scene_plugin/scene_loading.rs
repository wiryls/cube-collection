use bevy::prelude::*;

use super::{model::Seeds, SceneState};
use crate::plugin::loader_plugin::{LevelLoadingUpdated, LoadLevels, LoaderPlugin};

#[derive(Clone)]
pub struct HardReset();

pub fn setup(app: &mut App) {
    app.add_plugin(LoaderPlugin)
        .add_event::<HardReset>()
        .add_system(loading_enter.in_schedule(OnEnter(SceneState::Loading)))
        .add_system(
            loading_updated
                .run_if(in_state(SceneState::Loading))
                .run_if(on_event::<LevelLoadingUpdated>()),
        )
        .add_system(
            hard_reset
                .run_if(in_state(SceneState::Running))
                .run_if(on_event::<HardReset>()),
        );
}

fn loading_enter(mut commands: Commands) {
    commands.insert_resource(LoadLevels::new(r"level/index.toml"));
}

fn loading_updated(
    mut commands: Commands,
    mut events: EventReader<LevelLoadingUpdated>,
    mut progress: Local<(usize, usize)>,
    mut next_state: ResMut<NextState<SceneState>>,
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
                next_state.set(SceneState::Running);
            }
        }
    }
}

fn hard_reset(
    mut commands: Commands,
    mut events: EventReader<HardReset>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    while let Some(_) = events.iter().last() {
        next_state.set(SceneState::Loading);
        commands.insert_resource(LoadLevels::new(r"level/index.toml"));
    }
}
