use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// A Component which indicates the entity as a `Tile`, meaning it represents one part of the grid that stores the information of the cave's geography
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall { breaks: bool, ore: u8 },
    Ground { concealed: bool },

    // Convenience Tile, Should never see be seen in actual grids... only exist for comparison
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

    pub fn pattern_eq(&self, other: &Self) -> bool {
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

    pub fn is_walkable(&self) -> bool {
        match self {
            Tile::Ground { concealed: false } => true,
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
