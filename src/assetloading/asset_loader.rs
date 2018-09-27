// thanks to @jojolepro for his work at https://github.com/jojolepro/amethyst-extra
// licensed under apache 2.0 shown in LICENSE file

use std::collections::HashMap;

use amethyst::assets::*;
use amethyst::ecs::*;

pub struct AssetManager<T> {
    pub assets: HashMap<String, Handle<T>>,
}

impl<T> Default for AssetManager<T>
where T:Asset
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetManager<T>
where T:Asset
{
    pub fn new() -> Self {
        AssetManager {
            assets: HashMap::new(),
        }
    }

    pub fn get_asset_handle(&self, path: &str) -> Option<Handle<T>>
    {
        self.assets.get(path).cloned()
    }

    pub fn get_asset<'a>(&self, path: &str, storage: &'a AssetStorage<T>) -> Option<&'a T>
        where
            T: Asset,
    {
        if let Some(h) = self.get_asset_handle(path) {
            storage.get(&h)
        } else {
            None
        }
    }

    pub fn get_asset_handle_or_load<'a, F>(&mut self, path: &str, format: F, options: F::Options, storage: &'a mut AssetStorage<T>, loader: &Loader) -> Option< Handle<T>>
        where
            F: Format<T> + 'static,
    {
        if let Some(h) = self.get_asset_handle(path) {
            return Some(h)
        }
        if let Some(h) = self.load::<F>(path, format, options, storage, loader) {
            return Some(h)
        }
        None
    }

    pub fn load<F>(&mut self, path: &str, format: F, options: F::Options, storage: &mut AssetStorage<T>, loader: &Loader) -> Option<Handle<T>>
        where
            T: Asset,
            F: Format<T> + 'static,
    {
        if let Some(handle) = self.get_asset_handle(path) {
            return Some(handle);
        } else {
            let handle: Handle<T> = loader.load(path, format, options, (), storage);
            self.assets.insert(String::from(path), handle.clone());
            return Some(handle);
        }

        //TODO check if the asset exists...
    }

    /// Only removes the internal Handle<T>. To truly unload the asset, you need to drop all handles that you have to it.
    pub fn unload(&mut self, path: &str) {
        self.assets.remove(path);
    }
    //----------------------------------------------------------------------
}

impl<T> Component for AssetManager<T>
    where
        T: Asset,
{
    type Storage = VecStorage<Self>;
}