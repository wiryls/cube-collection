use bevy::prelude::*;
use cube_core::seed::Seed;

mod level;
mod loader;
use level::LevelSource;
use loader::LevelSeeds;

/// Use
///
/// ```
/// commands.insert_resource(LoadSeeds::new("INDEX_FILE_PATH"))
/// ```
///
/// to start loading.
pub struct LoaderPlugin;
impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_event::<LevelLoadingUpdated>()
            .add_systems(Update, load_levels.run_if(resource_exists::<LoadLevels>()))
            .register_asset_loader(loader::SeedsAssetLoader::default())
            .init_asset::<LevelSeeds>();
    }
}

#[derive(Clone, Event)]
pub enum LevelLoadingUpdated {
    Success { seeds: Vec<Seed> },
    Failure,
}

#[derive(Resource)]
pub struct LoadLevels(LoadLevelState);

impl LoadLevels {
    pub fn new<S: AsRef<str>>(index_file: S) -> Self {
        Self(LoadLevelState::Pending(String::from(index_file.as_ref())))
    }
}

enum LoadLevelState {
    Pending(String),
    Loading(Handle<LevelSeeds>),
}

fn load_levels(
    mut commands: Commands,
    mut status: ResMut<LoadLevels>,
    mut load_updated: EventWriter<LevelLoadingUpdated>,
    server: Res<AssetServer>,
    seeds: Res<Assets<LevelSeeds>>,
) {
    use bevy::asset::LoadState;
    match &mut status.as_mut().0 {
        LoadLevelState::Pending(path) => {
            let handle = server.load(&*path);
            status.0 = LoadLevelState::Loading(handle);
        }
        LoadLevelState::Loading(handle) => match server.load_state(&*handle) {
            LoadState::NotLoaded | LoadState::Loading => {}
            LoadState::Loaded if matches!(seeds.get(&*handle), Some(_)) => {
                let seeds = seeds.get(&*handle).cloned().unwrap().0;
                load_updated.send(LevelLoadingUpdated::Success { seeds });
                commands.remove_resource::<LoadLevels>();
            }
            _ => {
                load_updated.send(LevelLoadingUpdated::Failure);
                commands.remove_resource::<LoadLevels>();
            }
        },
    }
}
