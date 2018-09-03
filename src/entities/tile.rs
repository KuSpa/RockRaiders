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

use amethyst::config::Config;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
enum Tile {
    Wall {
        is_breakable: bool,
        contains_ore: u8,
    },
    Ground,
    //...
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    //already impls load(path) through trait impl
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            grid: vec![Vec::new()],
        }
    }
}
