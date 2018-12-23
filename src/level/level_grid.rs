const CONCEALED: &str = "concealed";

use amethyst::{
    core::{
        nalgebra::{Point2, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::{
        prelude::{Builder, Entity, World},
        storage::{GenericReadStorage, GenericWriteStorage},
    },
    renderer::{PngFormat, TextureMetadata},
};

use assetmanagement::util::*;
use entities::Tile;
use eventhandling::Clickable;
use level::TilePatternMap;
use pathfinding::directed::bfs;
use systems::{HoverHandler, Path};
use util;

/// A `Resource`, that holds every `Entity` that has a `Tile` Component and thus represents a part of the cave's layout
pub struct LevelGrid {
    /// A two-dimensional array of the cave's geography.
    grid: Vec<Vec<Entity>>,
}

impl LevelGrid {
    /// Instantiates the grid with `Entity`s that have a `Tile` component regarding to the given specification.
    ///
    /// Note, that this does not add `MeshHandles` or `Material` to the `Entity`, so the Entities won't get rendered yet.
    pub fn from_grid(mut tile_grid: Vec<Vec<Tile>>, world: &mut World) -> LevelGrid {
        let level_grid: Vec<Vec<Entity>> = tile_grid
            .iter_mut()
            .map(|tile_vec| {
                tile_vec
                    .iter_mut()
                    .map(|tile| {
                        world
                            .create_entity()
                            .with(*tile)
                            .with(GlobalTransform::default())
                            .build()
                    })
                    .collect()
            })
            .collect();

        LevelGrid { grid: level_grid }
    }

    /// Returns the following tiles(marked as X), if existing:
    /// ```-X-
    /// XOX
    /// -X-```
    pub fn direct_neighbors(&self, x: i32, y: i32) -> Vec<Entity> {
        let mut result = Vec::with_capacity(4);

        for (d_x, d_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            if let Some(entity) = self.get(x + d_x, y + d_y) {
                result.push(entity);
            }
        }
        result
    }

    /// Returns the following tiles(marked as X), if existing:
    /// ```X-X
    /// -O-
    /// X-X```
    pub fn diagonal_neighbors(&self, x: i32, y: i32) -> Vec<Entity> {
        let mut result = Vec::with_capacity(4);

        for (d_x, d_y) in [(1, -1), (1, 1), (-1, 1), (-1, -1)].iter() {
            if let Some(entity) = self.get(x + d_x, y + d_y) {
                result.push(entity);
            }
        }
        result
    }

    pub fn x_len(&self) -> usize {
        self.grid.len()
    }

    pub fn y_len(&self) -> usize {
        self.grid[0].len()
    }

    /// Uses the surrounding of a `TIle` to determine the `Mesh` that  fits into. For more information see `TilePatternConfig`
    pub fn determine_sprite_for<'a, T: GenericReadStorage<Component = Tile>>(
        &self,
        x: i32,
        y: i32,
        tile_patterns: &'a TilePatternMap,
        storage: &T,
    ) -> (&'a str, f32) {
        let tile = self.get_tile(x as i32, y as i32, storage).unwrap();
        if let Tile::Ground { concealed: true } = tile {
            return (CONCEALED, 0.);
        };
        let mut key = [[Tile::default(); 3]; 3];
        for delta_x in 0..3 {
            for delta_y in 0..3 {
                if let Some(t) =
                    self.get_tile(x + delta_x as i32 - 1, y + delta_y as i32 - 1, storage)
                {
                    key[delta_x][delta_y] = *t;
                    // if we get a None (aka out of bounds) we want a Tile::Default at this position.
                    // nothing to do here because of the initialize with Tile::Defaults...
                }
            }
        }

        for rotation in 0..4 {
            for (dict_key, value) in tile_patterns {
                let mut pattern_match = true;
                let dict_key = dict_key.iter().flatten();
                let key = key.iter().flatten();

                for (dict_tile, key_tile) in dict_key.zip(key) {
                    if !dict_tile.pattern_eq(key_tile) {
                        pattern_match = false;
                        break;
                    }
                }
                if pattern_match {
                    return (
                        value.as_str(),
                        (rotation as f32) * std::f32::consts::PI / 2.,
                    );
                }
            }
            key = util::rotate_3x3(&key);
        }
        panic!("Cannot determine sprite for: {:?}", util::rotate_3x3(&key));
    }

    /// determines and attaches the assets fitting the requested position the `Entity` at this position.
    /// From this point on, the entity should be rendered
    pub fn update_tile<
        T: GenericReadStorage<Component = Tile>,
        R: GenericWriteStorage<Component = Transform>,
        H: GenericWriteStorage<Component = HoverHandler>,
        C: GenericWriteStorage<Component = Box<dyn Clickable>>,
    >(
        &self,
        x: i32,
        y: i32,
        dict: &TilePatternMap,
        transforms: &mut R,
        tiles: &T,
        storages: &mut AssetStorages,
        hover_storage: &mut H,
        click_storage: &mut C,
    ) {
        let entity = self.get(x, y).unwrap();
        let (classifier, rotation) = self.determine_sprite_for(x, y, &dict, tiles);
        insert_into_asset_storages(entity, classifier, storages);

        let mut transform = Transform::default();
        transform.set_position(Vector3::new(x as f32, 0.0, y as f32));
        transform.rotate_local(Vector3::<f32>::y_axis(), -rotation);
        transforms.insert(entity, transform).unwrap();

        //Add hover handler for the Tile
        if self.get_tile(x, y, tiles).unwrap().is_walkable() {
            let (
                ref loader,
                ref _mesh_manager,
                ref _mesh_handles,
                ref _mesh_storage,
                ref mut tex_manager,
                ref _mat_storage,
                ref mut tex_storage,
                ref _default_mat,
            ) = storages;
            let hover_mat = tex_manager.get_asset_handle_or_load(
                "ground_hover",
                PngFormat,
                TextureMetadata::srgb(),
                tex_storage,
                &loader,
            );
            let handler = HoverHandler {
                hover: hover_mat,
                bounding_box: Tile::bounding_box(),
            };
            hover_storage.insert(entity, handler).unwrap();
            click_storage.insert(entity, Tile::click_handler()).unwrap();
        }
    }

    pub fn get_tile<'a, T: GenericReadStorage<Component = Tile>>(
        &self,
        x: i32,
        y: i32,
        storage: &'a T,
    ) -> Option<&'a Tile> {
        self.get(x, y).map(|entity| storage.get(entity).unwrap())
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        if x < 0 || y < 0 || x >= self.grid.len() as i32 {
            return None;
        }

        self.grid
            .get(x as usize)
            .unwrap()
            .get(y as usize)
            .map(|entity| *entity)
    }

    /// Determines a `Path` from the start to the end Entity. Returns `None` if there is no Path.
    pub fn find_path<
        T: GenericReadStorage<Component = Tile>,
        TR: GenericReadStorage<Component = Transform>,
    >(
        &self,
        start: Entity,
        destination: Entity,
        tiles: &T,
        transforms: &TR,
    ) -> Option<Path> {
        if let Some(result) = bfs::bfs(
            &start,
            |&entity| self.walkable_neighbors(&entity, tiles, transforms),
            |&node| node == destination,
        ) {
            let result = result
                .iter()
                .map(|entity| {
                    let position = self.grid_position_of(entity, transforms);
                    Point2::new(position.0 as f32, position.1 as f32)
                })
                .collect();

            return Some(Path { path: result });
        };

        None
    }

    /// Returns all direct neighbors that can be walked on
    pub fn walkable_neighbors<
        T: GenericReadStorage<Component = Tile>,
        TR: GenericReadStorage<Component = Transform>,
    >(
        &self,
        entity: &Entity,
        tiles: &T,
        transforms: &TR,
    ) -> Vec<Entity> {
        let mut possible_neighbors = Vec::<Entity>::with_capacity(8);
        let position = self.grid_position_of(entity, transforms);

        // Note that the diagonals are not included. At hte moment, the movement is not weighed, so a diagonal path is as expensive as a linear walking when going linear
        // thus, RockRaiders decide to go diagonal when they don't need to
        // TODO #22
        let mut directs = self.direct_neighbors(position.0, position.1);
        possible_neighbors.append(&mut directs);

        let result = possible_neighbors
            .iter()
            .filter_map(|&entity| {
                if tiles.get(entity).unwrap().is_walkable() {
                    return Some(entity);
                }
                None
            })
            .collect();

        result
    }

    /// Returns the position of the requested entity. Panics if the Entity is not part of the Grid
    fn grid_position_of<T: GenericReadStorage<Component = Transform>>(
        &self,
        entity: &Entity,
        storage: &T,
    ) -> (i32, i32) {
        if let Some(transform) = storage.get(*entity) {
            let x = transform.translation().x as i32;
            let y = transform.translation().z as i32;
            // Test if the entity is part of the Grid at all. `unwrap()` is okay, because if it not part of the grid, we panic anyway
            assert_eq!(*entity, self.get(x, y).unwrap());
            return (x, y);
        };
        panic!("Entity is not part of the grid, but its grid position was asked");
    }
}

impl Default for LevelGrid {
    fn default() -> LevelGrid {
        LevelGrid { grid: vec![vec![]] }
    }
}
