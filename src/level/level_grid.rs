const CONCEALED: &str = "concealed";

use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::storage::{GenericReadStorage, GenericWriteStorage};
use amethyst::prelude::*;
use entities::Tile;
use level::TilePatternMap;
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

    pub fn set_transform<T: GenericWriteStorage<Component = Transform>>(
        entity: Entity,
        x: i32,
        y: i32,
        rotation: i32,
        mut transforms: T,
    ) {
        let mut transform = Transform::default();
        transform.set_position(Vector3 {
            x: x as f32,
            y: 0.0,
            z: y as f32,
        });
        transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Deg(-rotation as f32));
        transforms.insert(entity, transform).unwrap();
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

    pub fn y_len_at(&self, x: usize) -> usize {
        self.grid[x].len()
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
}

impl Default for LevelGrid {
    fn default() -> LevelGrid {
        LevelGrid { grid: vec![vec![]] }
    }
}
