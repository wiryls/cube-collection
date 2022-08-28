use super::source::LoaderSource;
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    reflect::TypeUuid,
};
use serde::Deserialize;

#[derive(Default)]
pub struct TOMLSourceLoader;
impl AssetLoader for TOMLSourceLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let source = toml::from_slice::<LoaderSource>(bytes)?;
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
pub struct TOMLIndexLoader;
impl AssetLoader for TOMLIndexLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let data = toml::from_slice::<Index>(bytes)?;
            context.set_default_asset(LoadedAsset::new(data));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "664b2720-dcbe-11ec-9d64-0242ac120002"]
pub struct Index {
    pub directory: String,
    pub extension: String,
    pub name_list: Vec<String>,
}
