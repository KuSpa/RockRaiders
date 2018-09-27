// thanks to @jojolepro for his work at https://github.com/jojolepro/amethyst-extra
// licensed under apache 2.0 shown in LICENSE file

impl Component for AssetLoader {
    type Storage = VecStorage<Self>;
}

pub struct AssetManager<T> {
    pub assets: HashMap<String, Handle<T>>,
}

impl<T> Default for AssetManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetManager<T> {
    pub fn new() -> Self {
        AssetManager {
            assets: HashMap::new(),
        }
    }

    pub fn get_asset_handle<T>(&self, path: &str) -> Option<Handle<T>> {
        self.assets.get(path).cloned()
    }

    pub fn get_asset<'a, T>(&self, path: &str, storage: &'a AssetStorage<T>) -> Option<&'a T>
        where
            T: Asset,
    {
        if let Some(h) = self.get_asset_handle::<T>(path) {
            storage.get(&h)
        } else {
            None
        }
    }

    pub fn get_asset_or_load<'a, T, F>(&mut self, path: &str,format: F,options: F::Options, storage: &'a mut AssetStorage<T>, loader: &Loader ) -> Option<&'a T>
        where
            T: Asset,
            F: Format<T> + 'static,
    {
        if let Some(h) = self.get_asset_handle::<T>(path) {
            return storage.get(&h);
        }
        if let Some(h) = self.load::<T, F>(path, format, options,  storage, loader) {
            return storage.get(&h);
        }
        None
    }

    pub fn load<T, F>(&mut self, path: &str, format: F, options: F::Options,  storage: &mut AssetStorage<T>, loader: &Loader ) -> Option<Handle<T>>
        where
            T: Asset,
            F: Format<T> + 'static,
    {
        if let Some(handle) = self.get_asset_handle(path) {
            return Some(handle);
        }
        if let Some(p) = self.resolve_path(path) {
            let handle: Handle<T> = loader.load(p, format, options, (), storage);
            ali.assets.insert(String::from(path), handle.clone());
            return Some(handle);
        }
        None
    }

    /// Only removes the internal Handle<T>. To truly unload the asset, you need to drop all handles that you have to it.
    pub fn unload<T>(path: &str, ali: &mut AssetLoaderInternal<T>) {
        ali.assets.remove(path);
    }
    //----------------------------------------------------------------------
}

impl<T> Component for AssetManager<T>
    where
        T: Send + Sync + 'static,
{
    type Storage = VecStorage<Self>;
}