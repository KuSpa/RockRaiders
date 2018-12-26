use amethyst::{
    assets::*,
    renderer::{ObjFormat, PngFormat},
};
use std::collections::HashMap;

/// This trait provides basic information about the assets on disk
/// The data is stored in a dedicated folder per asset (e.g. `.png`, `.obj`, ...). Each of which has to be identical to the other asset folders except for the different endings
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

/// Amethyst may drop an asset as soon as there is no valid handle to the asset anymore. This causes problems when updating the asset of an entity to an asset, which has to be loaded from disk first.
///
/// Furthermore, when loosing a handle to an asset in Amethyst, it is required to load the asset once again to get another handle. So one's basically the exact same asset loaded twice.
///
/// To avoid both, this struct exists once per asset type `T` and holds a handle to every existing asset of type `T` and returns a copy of the requested handle. If the asset handle does not exist already, the asset is loaded from disk.
///
/// Note, that this disables automated asset dropping completely, because there will always be a valid handle to an asset, the one which is stored in this manager.
pub struct AssetManager<T> {
    /// A map of identifiers (the Path to the asset as `String`) to the asset handles
    pub assets: HashMap<String, Handle<T>>,
}

/// This looks broken. But it is exactly what we want. An assetmanager is a singleton in the system, since every resource can only exist once in the world (typesafe).
/// Whenever we compare with an other and and either is not zero, they are different, when both are zero, bot are Defaults
impl<T> PartialEq for AssetManager<T>
where
    T: Asset,
{
    fn eq(&self, other: &Self) -> bool {
        self.assets.len() == 0 && other.assets.len() == 0
    }
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

    /// returns the requested handle or, if non existent, load from disk vie the amethyst loader and return the newly gained handle.
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
        if let Some(h) = self.assets.get(path).cloned() {
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
