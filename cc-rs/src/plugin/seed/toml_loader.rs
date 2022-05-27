use super::toml_source::Source;
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    reflect::TypeUuid,
};
use serde::Deserialize;

#[derive(Default)]
pub struct SourceLoader;

impl AssetLoader for SourceLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let source = toml::from_slice::<Source>(bytes)?;
            let target = source.into_seed()?;
            context.set_default_asset(LoadedAsset::new(target));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.toml"]
    }
}

#[derive(Default)]
pub struct SourceListLoader;

impl AssetLoader for SourceListLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let data = toml::from_slice::<SourceList>(bytes)?;
            context.set_default_asset(LoadedAsset::new(data));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["index.toml"]
    }
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "664b2720-dcbe-11ec-9d64-0242ac120002"]
pub struct SourceList {
    directory: String,
    extension: String,
    name_list: Vec<String>,
}
