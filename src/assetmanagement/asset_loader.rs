// thanks to @jojolepro for his work at https://github.com/jojolepro/amethyst-extra
// licensed under apache 2.0 shown in LICENSE file

use amethyst::{
    assets::*,
    ecs::*,
    renderer::{ObjFormat, PngFormat},
};

use std::collections::HashMap;

pub trait AssetInformation {
    fn folder_name(&self) -> &'static str;
    fn extension(&self) -> &'static str;
}

impl AssetInformation for ObjFormat {
    fn folder_name(&self) -> &'static str {
        "meshes/"
    }
    fn extension(&self) -> &'static str {
        ".obj"
    }
}

impl AssetInformation for PngFormat {
    fn folder_name(&self) -> &'static str {
        "textures/"
    }
    fn extension(&self) -> &'static str {
        ".png"
    }
}

pub struct AssetManager<T> {
    pub assets: HashMap<String, Handle<T>>,
}

impl<T> Default for AssetManager<T>
where
    T: Asset,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetManager<T>
where
    T: Asset,
{
    pub fn new() -> Self {
        AssetManager {
            assets: HashMap::new(),
        }
    }

    pub fn get_asset_handle(&self, path: &str) -> Option<Handle<T>> {
        self.assets.get(path).cloned()
    }

    pub fn get_asset_handle_or_load<'a, F>(
        &mut self,
        path: &str,
        format: F,
        options: F::Options,
        storage: &'a mut AssetStorage<T>,
        loader: &Loader,
    ) -> Handle<T>
    where
        F: Format<T> + AssetInformation + 'static,
    {
        if let Some(h) = self.get_asset_handle(path) {
            return h;
        }

        let handle: Handle<T> = loader.load(
            format!("{}{}{}", format.folder_name(), path, format.extension()),
            format,
            options,
            (),
            storage,
        );
        self.assets.insert(String::from(path), handle.clone());
        handle
    }
}

impl<T> Component for AssetManager<T>
where
    T: Asset,
{
    type Storage = VecStorage<Self>;
}
