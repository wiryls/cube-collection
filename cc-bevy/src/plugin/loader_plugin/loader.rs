use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    reflect::TypeUuid,
};
use cc_core::seed::Seed;
use serde::Deserialize;

use super::LevelSource;

/////////////////////////////////////////////////////////////////////////////
// LevelSeed

#[derive(Clone, TypeUuid)]
#[uuid = "c99b1333-8ad3-4b26-a54c-7de542f43c51"]
pub struct LevelSeed(Seed);

impl LevelSeed {
    pub fn new(seed: Seed) -> Self {
        Self(seed)
    }
}

impl From<LevelSeed> for Seed {
    fn from(seed: LevelSeed) -> Self {
        seed.0
    }
}

#[derive(Default)]
pub struct TOMLLevelSeedLoader;
impl AssetLoader for TOMLLevelSeedLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let source = toml::from_slice::<LevelSource>(bytes)?;
            let target = source.into_seed()?;
            context.set_default_asset(LoadedAsset::new(target));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.toml"]
    }
}

/////////////////////////////////////////////////////////////////////////////
// LevelIndex

#[derive(Deserialize, TypeUuid)]
#[uuid = "664b2720-dcbe-11ec-9d64-0242ac120002"]
pub struct LevelList {
    pub directory: String,
    pub extension: String,
    pub name_list: Vec<String>,
}

#[derive(Default)]
pub struct TOMLLevelListLoader;
impl AssetLoader for TOMLLevelListLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let data = toml::from_slice::<LevelList>(bytes)?;
            context.set_default_asset(LoadedAsset::new(data));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
