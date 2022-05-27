mod toml_loader;
mod toml_source;
pub use toml_source::{Source, Error};

use bevy::prelude::{AddAsset, AssetServer, Commands, Handle, Local, Plugin, Res};

use crate::rule::seed::Seed;

pub struct LoaderPlugin;

#[derive(Default)]
pub struct Monitor {
    index: Handle<toml_loader::SourceList>,
    seeds: Vec<Handle<Seed>>,
}

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset_loader(toml_loader::SourceLoader::default())
            .add_asset_loader(toml_loader::SourceLoader::default())
            .add_startup_system(load_seed);
    }
}

fn load_seed(mut commands: Commands, mut monitor: Local<Monitor>, server: Res<AssetServer>) {
    monitor.index = server.load("assets/level/index.toml");

    // TODO: load seeds.
    _ = commands;
    _ = monitor.seeds;

    // if let Ok(handles) = server.load_folder("level") {
    //     let x = handles
    //         .into_iter()
    //         .map(|x| x.typed::<Seed>())
    //         .collect::<Vec<_>>();
    // }
}
