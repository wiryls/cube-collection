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
            .add_message::<LevelLoadingUpdated>()
            .add_systems(Update, load_levels.run_if(resource_exists::<LoadLevels>))
            .register_asset_loader(loader::SeedsAssetLoader)
            .init_asset::<LevelSeeds>();
    }
}

#[derive(Clone, Debug, Message)]
pub enum LevelLoadingUpdated {
    Success { seeds: Vec<Seed> },
    Failure,
}

#[derive(Resource, Debug)]
pub struct LoadLevels(LoadLevelState);

impl LoadLevels {
    pub fn new<S: AsRef<str>>(index_file: S) -> Self {
        Self(LoadLevelState::Pending(String::from(index_file.as_ref())))
    }
}

#[derive(Debug)]
enum LoadLevelState {
    Pending(String),
    Loading(Handle<LevelSeeds>),
}

fn load_levels(
    mut commands: Commands,
    mut status: ResMut<LoadLevels>,
    mut load_updated: MessageWriter<LevelLoadingUpdated>,
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
            LoadState::Loaded => {
                if let Some(data) = seeds.get(&*handle).cloned() {
                    let seeds = data.0;
                    load_updated.write(LevelLoadingUpdated::Success { seeds });
                    commands.remove_resource::<LoadLevels>();
                }
            }
            _ => {
                load_updated.write(LevelLoadingUpdated::Failure);
                commands.remove_resource::<LoadLevels>();
            }
        },
    }
}
