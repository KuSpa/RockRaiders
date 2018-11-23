const CONCEALED: &str = "concealed";

use amethyst::ecs::prelude::Entity;
use amethyst::ecs::storage::GenericReadStorage;
use amethyst::prelude::*;
use entities::Tile;
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
                    .map(|tile| world.create_entity().with(*tile).build())
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

    pub fn grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn determine_sprite_for<'a, T: GenericReadStorage<Component = Tile>>(
        &self,
        x: i32,
        y: i32,
        dictionary: &'a Vec<([[Tile; 3]; 3], String)>,
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
            for (dict_key, value) in dictionary {
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
