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

use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::{Component, Storage, DenseVecStorage};
use amethyst::prelude::*;
use util;
use amethyst::shred::Fetch;
use amethyst::ecs::storage::{MaskedStorage, GenericReadStorage};

//TODO impl From<Entity> Trait - less code in LevelGrid
#[derive(Clone, Copy, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall { breaks: bool, ore: u8 },
    Ground { concealed: bool },

    // Convenience Tiles, Should never see be seen in actual grids... only exist for comparison
    Any,
}

impl Tile {
    pub fn reveal(&mut self) {
        match self {
            Tile::Ground { concealed } => *concealed = false,
            _ => error!("Error revealing a Tile that cannot be revealed"),
        }
    }
}

// IMPORTANT - this is only implemented for the mesh selection, DO NOT USE IN OTHER CONTEXT
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

    /// breaks, if a concealed ground tile is directly next to a revealed ground tile... since this should never happen, we can ignore this case
    pub fn determine_sprite_for(
        &self,
        x: usize,
        y: usize,
        dictionary: &Vec<([[Tile; 3]; 3], String)>,
    ) -> (String, i32) {
        match self.get(x as i32, y as i32) {
            Tile::Ground { concealed: true } => return ("concealed".to_string(), 0), // If the grond itself is concealed, it stays concealed.. no need for extra logic
            _ => {
                let mut key = [[Tile::default(); 3]; 3];
                for delta_x in 0..3 {
                    for delta_y in 0..3 {
                        key[delta_x][delta_y] =
                            self.get((x + delta_x) as i32 - 1, (y + delta_y) as i32 - 1);
                    }
                }

                for rotation in 0..4 {
                    if let Some(result) = util::find_in_vec(&key, &dictionary) {
                        debug!("{:?} was found", result);
                        return (result.clone(), 90 * (rotation + 1));
                    };
                    key = util::rotate_3x3(&key);
                }
                panic!("Cannot determine sprite for: {:?}", util::rotate_3x3(&key));
            }
        }
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

        *self.grid[x].get(y as usize).unwrap_or(&Tile::default())
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
        let mut result = vec![];

        for (d_x, d_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            if let Some(entity) = self.get(x + d_x, y + d_y) {
                result.push(entity);
            }
        }
        result
    }

    pub fn diagonal_neighbors(&self, x: i32, y: i32) -> Vec<Entity> {
        let mut result = vec![];

        for d_x in [-1, 1].iter() {
            for d_y in [-1, 1].iter() {
                if let Some(entity) = self.get(x + d_x, y + d_y) {
                    result.push(entity);
                }
            }
        }
        result
    }


    pub fn grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn determine_sprite_for<T: GenericReadStorage>(&self, x: usize, y: usize, dict: &Vec<([[Tile; 3]; 3], String)>, tile_storage: &T) -> (String, i32)
        where <T as GenericReadStorage>::Component: Clone,
              std::vec::Vec<Tile>: std::iter::FromIterator<<T as GenericReadStorage>::Component> {
        let grid = self.generate_tile_grid_copy::<T>(tile_storage);
        grid.determine_sprite_for(x, y, dict)
    }

    // we cannot store and use the Grid we deserialized, because it may have changed and we don't want to have two representations of the the same Grid
    fn generate_tile_grid_copy<T: GenericReadStorage>(&self, tile_storage: &T) -> Grid
        where <T as GenericReadStorage>::Component: Clone,
              std::vec::Vec<Tile>: std::iter::FromIterator<<T as GenericReadStorage>::Component> {
        let mut grid = self.grid.clone();
        Grid {
            grid: grid
                .iter_mut()
                .map(|vec| {
                    vec.iter_mut()
                        .map(|entity|
                            (*tile_storage.get(*entity).unwrap()).clone()).collect()
                })
                .collect(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.grid.len() {
            return None;
        }

        Some(self.grid[x][y])
    }
}

impl Default for LevelGrid {
    fn default() -> LevelGrid {
        LevelGrid {
            grid: vec![Vec::new()],
        }
    }
}