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
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use util;

//TODO impl From<Entity> Trait - less code in LevelGrid
#[derive(Clone, Copy, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall { breaks: bool, ore: u8 },
    Ground,

    // Convenience Tiles, Should never see be seen in actual grids... only exist for comparison
    Any,
}

//In order to use generic Tiles like the AnyTile, we need to change the equality function of the Tile enum, so that it actually matches every Tile
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (other, self) {
            (Tile::Wall { .. }, Tile::Wall { .. }) => true,
            (Tile::Ground, Tile::Ground) => true,
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

    pub fn determine_sprite_for(
        &self,
        x: usize,
        y: usize,
        dictionary: &Vec<([[Tile; 3]; 3], String)>,
    ) -> (String, i32) {
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

    // TODO adapt x,y to let (0,0) on the "bottom left" of the array
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

    pub fn grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn determine_sprite_for(&self, x: usize, y: usize, world: &World) -> (String, i32) {
        let dict = world.read_resource::<Vec<([[Tile; 3]; 3], String)>>();

        let grid = self.generate_tile_grid_copy(world);
        grid.determine_sprite_for(x, y, &dict)
    }

    // we cannot store and use the Grid we deserialized, because it may have changed and we don't want to have two representations of the the same Grid
    fn generate_tile_grid_copy(&self, world: &World) -> Grid {
        let mut grid = self.grid.clone();
        Grid {
            grid: grid
                .iter_mut()
                .map(|vec| {
                    vec.iter_mut()
                        .map(|entity| (*world.read_storage::<Tile>().get(*entity).unwrap()).clone())
                        .collect()
                })
                .collect(),
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
