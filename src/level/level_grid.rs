const CONCEALED: &str = "concealed";

use amethyst::core::cgmath::{Deg, Point2, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::storage::{GenericReadStorage, GenericWriteStorage};
use amethyst::prelude::*;
use assetmanagement::util::*;
use entities::Tile;
use level::TilePatternMap;
use pathfinding::directed::bfs;
use systems::MovementIntent;
use util;

pub struct LevelGrid {
    grid: Vec<Vec<Entity>>,
}

impl LevelGrid {
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

    pub fn direct_neighbors(&self, x: i32, y: i32) -> Vec<Entity> {
        let mut result = Vec::with_capacity(4);

        for (d_x, d_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            if let Some(entity) = self.get(x + d_x, y + d_y) {
                result.push(entity);
            }
        }
        result
    }

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

    pub fn determine_sprite_for<'a, T: GenericReadStorage<Component = Tile>>(
        &self,
        x: i32,
        y: i32,
        tile_patterns: &'a TilePatternMap,
        storage: &T,
    ) -> (&'a str, i32) {
        let tile = self.get_tile(x as i32, y as i32, storage).unwrap();
        if let Tile::Ground { concealed: true } = tile {
            return (CONCEALED, 0);
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
                    return (value.as_str(), 90 * rotation);
                }
            }
            key = util::rotate_3x3(&key);
        }
        panic!("Cannot determine sprite for: {:?}", util::rotate_3x3(&key));
    }

    pub fn update_tile<
        T: GenericReadStorage<Component = Tile>,
        R: GenericWriteStorage<Component = Transform>,
    >(
        &self,
        x: i32,
        y: i32,
        dict: &TilePatternMap,
        transforms: &mut R,
        tiles: &T,
        storages: &mut AssetStorages,
    ) {
        let entity = self.get(x, y).unwrap();
        let (classifier, rotation) = self.determine_sprite_for(x, y, &dict, tiles);
        insert_into_asset_storages(entity, classifier, storages);

        let mut transform = Transform::default();
        transform.set_position(Vector3 {
            x: x as f32,
            y: 0.0,
            z: y as f32,
        });
        transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(-rotation as f32));
        transforms.insert(entity, transform).unwrap();
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

    pub fn find_path<
        T: GenericReadStorage<Component = Tile>,
        TR: GenericReadStorage<Component = Transform>,
    >(
        &self,
        start: Entity,
        destination: Entity,
        tiles: &T,
        transforms: &TR,
    ) -> Option<MovementIntent> {
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

            return Some(MovementIntent { path: result });
        };

        None
    }

    fn walkable_neighbors<
        T: GenericReadStorage<Component = Tile>,
        TR: GenericReadStorage<Component = Transform>,
    >(
        &self,
        entity: &Entity,
        tiles: &T,
        transforms: &TR,
    ) -> Vec<Entity> {
        let mut result = Vec::<Entity>::with_capacity(8);
        let position = self.grid_position_of(entity, transforms);

        // Note that the diagonals are not included. At hte moment, the movement is not weighed, so a diagonal path is as expensive as a linear walking when going linear
        // thus, RockRaiders decide to go diagonal when they don't need to
        // TODO #22
        let mut directs = self.direct_neighbors(position.0, position.1);
        result.append(&mut directs);

        let result = result
            .iter()
            .filter(|&entity| {
                if let Some(result) = tiles.get(*entity) {
                    return result.is_walkable();
                };
                false
            })
            .map(|entity| *entity)
            .collect::<Vec<Entity>>();

        result
    }

    fn grid_position_of<T: GenericReadStorage<Component = Transform>>(
        &self,
        entity: &Entity,
        storage: &T,
    ) -> (i32, i32) {
        if let Some(transform) = storage.get(*entity) {
            return (
                transform.translation[0] as i32,
                transform.translation[2] as i32,
            );
        };
        panic!("Entity is not part of the grid, but its grid position was asked");
    }
}

impl Default for LevelGrid {
    fn default() -> LevelGrid {
        LevelGrid { grid: vec![vec![]] }
    }
}
