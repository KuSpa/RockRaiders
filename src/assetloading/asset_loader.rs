// thanks to @jojolepro for his work at https://github.com/jojolepro/amethyst-extra
// licensed under apache 2.0 shown in LICENSE file

use std::collections::HashMap;

use amethyst::assets::*;
use amethyst::ecs::*;
use amethyst::renderer::{Mesh, Texture};


pub trait AssetInformation {
    fn folder_name() -> String;
    fn extension() -> String;
}

impl AssetInformation for Mesh {
    fn folder_name() -> String { "meshes/".to_string() }
    fn extension() -> String { ".obj".to_string() }
}

impl AssetInformation for Texture {
    fn folder_name() -> String { "textures/".to_string()}
        fn extension() -> String { ".png".to_string() }
}


pub struct AssetManager<T> {
    pub assets: HashMap<String, Handle<T>>,
}

impl<T> Default for AssetManager<T>
    where
        T: Asset + AssetInformation,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetManager<T>
    where
        T: Asset + AssetInformation,
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
            F: Format<T> + 'static,
    {
        if let Some(h) = self.get_asset_handle(path) {
            return h;
        }

        let handle: Handle<T> = loader.load(format!("{}{}{}",T::folder_name(),path, T::extension()), format, options, (), storage);
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
