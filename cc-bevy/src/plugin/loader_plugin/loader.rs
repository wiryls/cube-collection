use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    reflect::TypeUuid,
};
use cc_core::seed::Seed;
use serde::Deserialize;

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

/////////////////////////////////////////////////////////////////////////////
// LevelIndex

#[derive(Deserialize, TypeUuid)]
#[uuid = "664b2720-dcbe-11ec-9d64-0242ac120002"]
pub struct LevelList {
    pub directory: String,
    pub extension: String,
    pub name_list: Vec<String>,
}

/////////////////////////////////////////////////////////////////////////////
// Loader

#[derive(Default)]
pub struct TOMLAssetLoader;
impl AssetLoader for TOMLAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            use super::LevelSource;
            use toml::Value;

            let value = toml::from_slice::<Value>(bytes)?;
            if let Value::Table(table) = &value {
                if table.contains_key("map") {
                    // level souce file
                    let source = value.try_into::<LevelSource>()?;
                    let target = source.into_seed()?;
                    context.set_default_asset(LoadedAsset::new(target));
                } else {
                    // level index file
                    let source = value.try_into::<LevelList>()?;
                    context.set_default_asset(LoadedAsset::new(source));
                }
            }
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
