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
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::Entity;

//TODO impl From<Entity> Trait - less code in LevelGrid
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall {
        is_breakable: bool,
        contains_ore: u8,
    },
    Ground,
    // it may be smart to add a NullObject pattern (SWA FTW) for some cases like `is_breakable`
    // in order to reduce many double if clauses...
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

        let mut x: i32 = -1;
        let mut y: i32 = -1;

        // at the moment, we add the transform here, because its less code to do it with the builder, than to acces the storage and insert it manually ;)
        // TODO refactor - add to next loop
        let level_grid: Vec<Vec<Entity>> = tile_grid.iter_mut().map(
            |tile_vec| {
                x += 1;
                y = -1;
                tile_vec.iter_mut().map(
                    |tile| {
                        y += 1;
                        entity_from_tile(*tile, x, y, world)
                    }).collect()
            }).collect();

        LevelGrid { grid: level_grid }
    }

    pub fn grid(&self) -> &Vec<Vec<Entity>> {
        &self.grid
    }

    pub fn determine_sprite_for(&self, x: usize, y: usize, world: &World) -> (i32, i32) {
        let grid = self.generate_tile_grid_copy(world);


        // TODO add brain here
        (0,0)
//     /(  )\
//       L L
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


fn entity_from_tile(tile: Tile, x: i32, y: i32, world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_position(Vector3 { x: x as f32, y: 0.0, z: -y as f32 });
    world
        .create_entity()
        .with(tile)
        .with(GlobalTransform::default())
        .with(transform)
        .build()
}

//impl Serialize for LevelGrid {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
//        S: Serializer, {
//        //TODO
//    }
//}

