use bevy::{asset::AssetPath, prelude::*};
use cc_core::seed::Seed;
use iyes_loopless::prelude::*;

mod level;
mod loader;
use level::LevelSource;
use loader::{LevelList, LevelSeed};

/// Use
///
/// ```
/// commands.insert_resource(LoadSeeds::new("INDEX_PATH"))
/// ```
///
/// to start loading.
pub struct LoaderPlugin;
impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_event::<LevelLoadingUpdated>()
            .add_asset_loader(loader::TOMLAssetLoader::default())
            .add_asset::<LevelList>()
            .add_asset::<LevelSeed>()
            .add_system(load_levels.run_if_resource_exists::<LoadLevels>());
    }
}

#[derive(Clone)]
pub enum LevelLoadingUpdated {
    Loading { total: usize, done: usize },
    Failure { which: String },
    Success { seeds: Vec<Seed> },
}

pub struct LoadLevels(LoadLevelState);

impl LoadLevels {
    pub fn new<S: AsRef<str>>(index_file: S) -> Self {
        Self(LoadLevelState::Pending(String::from(index_file.as_ref())))
    }
}

enum LoadLevelState {
    Pending(String),
    Finding(Handle<LevelList>),
    Loading(Vec<Handle<LevelSeed>>),
    Success(Vec<Seed>),
    Failure(String),
}

impl Default for LoadLevelState {
    fn default() -> Self {
        Self::Pending(String::new())
    }
}

fn load_levels(
    mut commands: Commands,
    mut status: ResMut<LoadLevels>,
    mut load_updated: EventWriter<LevelLoadingUpdated>,
    server: Res<AssetServer>,
    index_assets: Res<Assets<LevelList>>,
    seeds_assets: Res<Assets<LevelSeed>>,
) {
    use bevy::asset::LoadState;
    match &mut status.as_mut().0 {
        LoadLevelState::Pending(path) => {
            status.0 = LoadLevelState::Finding(server.load(&*path));
        }

        LoadLevelState::Finding(handle) => match server.get_load_state(&*handle) {
            LoadState::Loading | LoadState::NotLoaded => {}
            LoadState::Loaded => {
                status.0 = if let Some(index) = index_assets.get(&*handle) {
                    let dir = std::path::Path::new(&index.directory);
                    let out = index
                        .name_list
                        .iter()
                        .map(|x| dir.join([x, ".", &index.extension].concat()))
                        .map(|x| server.load(x))
                        .collect();

                    LoadLevelState::Loading(out)
                } else {
                    server.get_handle_path(&*handle).into()
                }
            }
            _ => {
                status.0 = server.get_handle_path(&*handle).into();
            }
        },

        LoadLevelState::Loading(handles) => {
            let mut done = 0;
            let total = handles.len();
            let fail = handles
                .iter()
                .find_map(|handle| match server.get_load_state(handle) {
                    LoadState::Loading | LoadState::NotLoaded => None,
                    LoadState::Loaded => {
                        done += 1;
                        None
                    }
                    _ => Some(server.get_handle_path(handle)),
                });

            if let Some(which) = fail {
                status.0 = which.into();
            } else if done == total {
                let output: Vec<Seed> = handles
                    .iter()
                    .filter_map(|x| seeds_assets.get(x))
                    .map(|x| x.clone().into())
                    .collect::<Vec<_>>();
                status.0 = LoadLevelState::Success(output);
            } else {
                load_updated.send(LevelLoadingUpdated::Loading { total, done });
            }
        }

        LoadLevelState::Success(output) => {
            let mut seeds = Vec::new();
            std::mem::swap(&mut seeds, output);

            let event = LevelLoadingUpdated::Success { seeds };
            load_updated.send(event);
            commands.remove_resource::<LoadLevels>();
        }

        LoadLevelState::Failure(output) => {
            let mut which = String::new();
            std::mem::swap(&mut which, output);

            let event = LevelLoadingUpdated::Failure { which };
            load_updated.send(event);
            commands.remove_resource::<LoadLevels>();
        }
    }
}

impl<'a> From<Option<AssetPath<'a>>> for LoadLevelState {
    fn from(path: Option<AssetPath>) -> Self {
        let which = path
            .map(|x| String::from(x.path().to_string_lossy()))
            .unwrap_or_default();
        LoadLevelState::Failure(which)
    }
}
