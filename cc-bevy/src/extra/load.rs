mod loader;
mod source;
pub use source::{Error, Source};

use crate::model::seed;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

/// Use
///
/// ```
/// commands.insert_resource(LoadSeeds::new("INDEX_PATH"))
/// ```
///
/// to start loading ```Res<seed::Seeds>```.
pub struct LoaderPlugin;
impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadSeedsUpdated>()
            .add_asset_loader(loader::TOMLSourceLoader::default())
            .add_asset_loader(loader::TOMLIndexLoader::default())
            .add_asset::<loader::Index>()
            .add_asset::<seed::Seed>()
            .add_system(load_seeds.run_if_resource_exists::<LoadSeeds>());
    }
}

#[derive(Clone)]
pub enum LoadSeedsUpdated {
    Loading { total: i32, done: i32 },
    Failure { which: String },
}

pub struct LoadSeeds(LoadSeedsState);

impl LoadSeeds {
    pub fn new<S: AsRef<str>>(index: S) -> Self {
        Self(LoadSeedsState::Pending(String::from(index.as_ref())))
    }
}

enum LoadSeedsState {
    Pending(String),
    Finding(Handle<loader::Index>),
    Loading(Vec<Handle<seed::Seed>>),
    Stopped,
}

impl Default for LoadSeedsState {
    fn default() -> Self {
        Self::Pending(String::new())
    }
}

fn load_seeds(
    mut commands: Commands,
    mut status: ResMut<LoadSeeds>,
    mut progress: Local<(i32, i32)>,
    mut load_updated: EventWriter<LoadSeedsUpdated>,
    server: Res<AssetServer>,
    index_assets: Res<Assets<loader::Index>>,
    seeds_assets: Res<Assets<seed::Seed>>,
) {
    use bevy::asset::LoadState;
    match &status.0 {
        LoadSeedsState::Pending(path) => {
            status.0 = LoadSeedsState::Finding(server.load(path));
        }

        LoadSeedsState::Finding(handle) => match server.get_load_state(handle) {
            LoadState::Loading => {}
            LoadState::Loaded if index_assets.get(handle).is_some() => {
                let index = index_assets.get(handle).unwrap();
                let dir = std::path::Path::new(&index.directory);
                let out = index
                    .name_list
                    .iter()
                    .map(|x| dir.join([x, ".", &index.extension].concat()))
                    .map(|x| server.load(x))
                    .collect();
                status.0 = LoadSeedsState::Loading(out);
            }
            _ => {
                let which = server
                    .get_handle_path(handle)
                    .map(|x| String::from(x.path().to_string_lossy()))
                    .unwrap_or_default();
                let event = LoadSeedsUpdated::Failure { which };
                load_updated.send(event);
                status.0 = LoadSeedsState::Stopped {};
            }
        },

        LoadSeedsState::Loading(handles) => {
            let total = handles.len() as i32;
            let mut done = 0;
            let mut fail = None;

            for handle in handles {
                match server.get_load_state(handle) {
                    LoadState::Loading => {}
                    LoadState::Loaded => done += 1,
                    _ if fail.is_none() => {
                        fail = Some(
                            server
                                .get_handle_path(handle)
                                .map(|x| String::from(x.path().to_string_lossy()))
                                .unwrap_or_default(),
                        )
                    }
                    _ => {}
                }
            }

            if let Some(which) = fail {
                load_updated.send(LoadSeedsUpdated::Failure { which });
                status.0 = LoadSeedsState::Stopped {};
            } else if done == total {
                let output: seed::Seeds = handles
                    .iter()
                    .filter_map(|x| seeds_assets.get(x))
                    .map(|x| x.clone())
                    .collect::<Vec<_>>()
                    .into();
                commands.insert_resource(output);
                load_updated.send(LoadSeedsUpdated::Loading { total, done });
                status.0 = LoadSeedsState::Stopped {};
            } else if progress.0 != total || progress.1 != done {
                *progress = (total, done);
                load_updated.send(LoadSeedsUpdated::Loading { total, done });
            }
        }

        LoadSeedsState::Stopped => {
            *progress = (0, 0);
            commands.remove_resource::<LoadSeeds>();
        }
    };
}
