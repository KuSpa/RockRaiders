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

const CONCEALED: &str = "concealed";

use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::ecs::storage::GenericReadStorage;
use amethyst::prelude::*;
use util;

//TODO impl From<Entity> Trait - less code in LevelGrid
#[derive(Clone, Copy, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall { breaks: bool, ore: u8 },
    Ground { concealed: bool },

    // Convenience Tiles, Should never see be seen in actual grids... only exist for comparison
    Any,
}

impl Tile {
    pub fn reveal(&mut self) -> bool {
        match self {
            Tile::Ground { concealed } => {
                if *concealed {
                    *concealed = false;
                    return true;
                }
            }
            _ => (),
        };
        false
    }
}

// IMPORTANT - this is only implemented for the mesh selection, DO NOT USE IN OTHER CONTEXT
// TODO implement pattern_eq in tile
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (other, self) {
            (Tile::Wall { .. }, Tile::Wall { .. }) => true, // a Wall is a Wall
            (Tile::Ground { concealed: false }, Tile::Ground { concealed: false }) => true, // Ground is Ground, when it was revealed
            (Tile::Ground { concealed: true }, Tile::Wall { .. }) => true, //concealed Ground is hidden as wall mesh
            (Tile::Wall { .. }, Tile::Ground { concealed: true }) => true,
            (Tile::Any, _) => true,
            (_, Tile::Any) => true,
            _ => false,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Wall {
            breaks: false,
            ore: 0,
        }
    }
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

    pub fn determine_sprite_for<'a>(
        &self,
        x: i32,
        y: i32,
        dictionary: &'a Vec<([[Tile; 3]; 3], String)>,
    ) -> (&'a str, i32) {
        let tile = self.get(x as i32, y as i32);
        if let Tile::Ground { concealed: true } = tile {
            return (CONCEALED, 0);
        };
        let mut key = [[Tile::default(); 3]; 3];
        for delta_x in 0..3 {
            for delta_y in 0..3 {
                key[delta_x][delta_y] = self.get(x + delta_x as i32 - 1, y + delta_y as i32 - 1);
            }
        }

        for rotation in 0..3 {
            if let Some(result) = util::find_in_vec(&key, &dictionary) {
                return (result.as_str(), 90 * (rotation + 1));
            };
            key = util::rotate_3x3(&key);
        }
        panic!("Cannot determine sprite for: {:?}", util::rotate_3x3(&key));
    }

    fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 {
            return Tile::default();
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.grid.len() {
            return Tile::default();
        }

        *self.grid[x].get(y).unwrap_or(&Tile::default())
    }
}
impl Default for Grid {
    fn default() -> Grid {
        Grid { grid: vec![vec![]] }
    }
}

pub struct LevelGrid {
    grid: Vec<Vec<Entity>>,
}

impl LevelGrid {
    pub fn from_grid(grid: Grid, world: &mut World) -> LevelGrid {
        let mut tile_grid = grid.clone_grid();

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

    // we cannot store and use the Grid we deserialized, because it may have changed and we don't want to have two representations of the the same Grid
    pub fn generate_tile_grid_copy<T: GenericReadStorage<Component = Tile>>(
        &self,
        tile_storage: &T,
    ) -> Grid {
        let mut grid = self.grid.clone();
        Grid {
            grid: grid
                .iter_mut()
                .map(|vec| {
                    vec.iter_mut()
                        .map(|entity| (*tile_storage.get(*entity).unwrap()).clone())
                        .collect()
                })
                .collect(),
        }
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
        LevelGrid {
            grid: vec![Vec::new()],
        }
    }
}
