//#############################
//# The Dream - use an enum   #
//#############################

// https://users.rust-lang.org/t/could-enums-be-considered-an-anti-pattern/10068
// https://internals.rust-lang.org/t/impl-trait-for-enum-variant/4131

//pub enum Tile {
//    Ground,
//    Wall,
//}
//
//trait Tiled {
//    type T;
//    fn new() -> Self::T;
//}
//
//impl Tiled for Tile::Ground {
//    type T = Tile::Ground;
//
//    fn new() -> Tile::Ground {
//        Tile::Ground {}
//    }
//}

use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use std::collections::HashMap;
use amethyst::core::cgmath::Vector3;
use amethyst::ecs::prelude::Entity;

//TODO impl From<Entity> Trait - less code in LevelGrid
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall {
        is_breakable: bool,
        contains_ore: u8,
    },
    Ground,
    None
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
}


impl Grid {
    pub fn clone_grid(&self) -> Vec<Vec<Tile>> {
        self.grid.clone()
    }


    pub fn determine_sprite_for(&self, x: usize, y: usize, dictionary: HashMap<[[Tile; 3]; 3], (String, i32)>) -> (String, i32) {
        let mut key = [[Tile::None; 3]; 3];
        for delta_x in 0..3 {
            for delta_y in 0..3 {
                key[delta_x][delta_y] = self.get((x + delta_x) as i32 - 1, (y + delta_y) as i32 - 1);
            }
        };

        (*dictionary.get(&key).expect(&format!("unkown Tile pattern: {:?}", key))).clone()
    }


    fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 {
            return Tile::None;
        }

        let x = x as usize;
        let y = y as usize;

        if x > self.grid.len()
            {
                return Tile::None;
            }

        *self.grid[x].get(y as usize).unwrap_or(&Tile::None)
    }
}


impl Default for Grid {
    fn default() -> Grid {
        Grid {
            grid: vec![Vec::new()],
        }
    }
}


pub struct LevelGrid {
    grid: Vec<Vec<Entity>>,
}

impl LevelGrid {
    pub fn from_grid(grid: Grid, world: &mut World) -> LevelGrid {
        let mut tile_grid = grid.clone_grid();

        let level_grid: Vec<Vec<Entity>> = tile_grid.iter_mut().map(
            |tile_vec| {
                tile_vec.iter_mut().map(
                    |tile| {
                        world
                            .create_entity()
                            .with(*tile)
                            .build()
                    }).collect()
            }).collect();

        LevelGrid { grid: level_grid }
    }

    pub fn grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn determine_sprite_for(&self, x: usize, y: usize, world: &World) -> (String, i32) {
        // TODO create ron file
        // deserialize
        let dict = HashMap::<[[Tile; 3]; 3], (String, i32)>::new();
        let grid = self.generate_tile_grid_copy(world);
        grid.determine_sprite_for(x, y, dict)
    }


    // we cannot store and use the Grid we deserialized, because it may have changed and we don't want to have two representations of the the same Grid
    fn generate_tile_grid_copy(&self, world: &World) -> Grid {
        let mut grid = self.grid.clone();
        Grid {
            grid: grid.iter_mut().map(
                |vec|
                    vec.iter_mut().map(
                        |entity|
                            (*world.read_storage::<Tile>().get(*entity).unwrap()).clone()
                    ).collect()
            ).collect()
        }
    }


    pub fn get(&self, x: usize, y: usize) -> Entity {
        *self.grid.get(x).unwrap().get(y).unwrap()
    }
}

//impl Serialize for LevelGrid {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
//        S: Serializer, {
//        //TODO
//    }
//}

