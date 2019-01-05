use amethyst::{
    assets::{AssetStorage, Loader},
    config::Config,
    renderer::{
        Material, Mesh, MeshHandle, ObjFormat, PngFormat, Texture, TextureData, TextureHandle,
        TextureMetadata,
    },
};
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct MeshManager {
    assets: HashMap<String, MeshHandle>,
}

impl MeshManager {
    pub fn get_handle_or_load<'a>(
        &mut self,
        path: &str,
        loader: &Loader,
        storage: &'a mut AssetStorage<Mesh>,
    ) -> MeshHandle {
        if let Some(handle) = self.assets.get(path).cloned() {
            return handle;
        }

        let handle = loader.load(Self::sanitize_path(&path), ObjFormat, (), (), storage);
        self.assets.insert(String::from(path), handle.clone());
        handle
    }

    /// Adds the foldername and file extension to the assets' name
    #[inline(always)]
    fn sanitize_path(path: &str) -> String {
        format!("{}{}{}", "meshes/", path, ".obj")
    }

    pub fn is_default(&self) -> bool {
        self.assets.len() == 0
    }
}

impl PartialEq for MeshManager {
    fn eq(&self, other: &Self) -> bool {
        self.is_default() && other.is_default()
    }
}

#[derive(Default)]
pub struct TextureManager {
    assets: HashMap<String, TextureHandle>,
    colors: HashMap<String, [f32; 4]>,
    defaults: Option<Material>,
}

impl TextureManager {
    pub fn material_from<'a>(
        &mut self,
        path: &str,
        loader: &Loader,
        mut storage: &'a mut AssetStorage<Texture>,
    ) -> Material {
        Material {
            albedo: self.get_handle_or_load(path, &loader, &mut storage),
            ..self.defaults.clone().unwrap()
        }
    }

    pub fn get_handle_or_load<'a>(
        &mut self,
        path: &str,
        loader: &Loader,
        storage: &'a AssetStorage<Texture>,
    ) -> TextureHandle {
        if let Some(handle) = self.assets.get(path).cloned() {
            return handle;
        }

        if let Some(color) = self.colors.get(path) {
            let handle = loader.load_from_data(TextureData::color(color.clone()), (), &storage);
            self.assets.insert(String::from(path), handle.clone());
            return handle;
        };

        let handle = loader.load(
            Self::sanitize_path(&path),
            PngFormat,
            TextureMetadata::srgb(),
            (),
            storage,
        );
        self.assets.insert(String::from(path), handle.clone());
        handle
    }

    /// Adds the foldername and file extension to the assets' name
    #[inline(always)]
    fn sanitize_path(path: &str) -> String {
        format!("{}{}{}", "textures/", path, ".png")
    }

    pub fn initialize_with(&mut self, default: Material) {
        self.defaults = Some(default);
        // TODO
        self.colors = HashMap::<String, [f32; 4]>::load(Path::new(&format!(
            "{}/resources/color_palette.ron",
            env!("CARGO_MANIFEST_DIR")
        )));
    }

    pub fn is_default(&self) -> bool {
        self.assets.len() == 0
    }
}

impl PartialEq for TextureManager {
    fn eq(&self, other: &Self) -> bool {
        self.is_default() && other.is_default()
    }
}

pub mod util {
    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Entity, Write, WriteStorage},
        renderer::{Material, Mesh, MeshHandle, Texture},
    };
    use assetmanagement::{MeshManager, TextureManager};

    pub type TextureStorages<'a> = (
        Write<'a, TextureManager>,
        Write<'a, AssetStorage<Texture>>,
        WriteStorage<'a, Material>,
    );
    pub type MeshStorages<'a> = (
        Write<'a, MeshManager>,
        Write<'a, AssetStorage<Mesh>>,
        WriteStorage<'a, MeshHandle>,
    );

    pub fn attach_assets(
        entity: Entity,
        path: &str,
        loader: &Loader,
        (ref mut texture_manager, ref mut texture_storage, ref mut material_storage): &mut TextureStorages,
        (ref mut mesh_manager, ref mut mesh_storage, ref mut mesh_handle_storage): &mut MeshStorages,
    ) {
        let mesh = mesh_manager.get_handle_or_load(&path, &loader, mesh_storage);
        let material = texture_manager.material_from(path, &loader, texture_storage);

        material_storage.insert(entity, material).unwrap();
        mesh_handle_storage.insert(entity, mesh).unwrap();
    }
}
