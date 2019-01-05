use amethyst::{
    assets::{AssetStorage, Loader},
    core::{nalgebra::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity, World},
    renderer::Texture,
};

use assetmanagement::TextureManager;
use eventhandling::{ClickHandlerComponent, Clickable, HoverHandlerComponent, SimpleHoverHandler};
use level::{LevelGrid, SelectedRockRaider};
use ncollide3d::shape::Cuboid;

/// A Component which indicates the entity as a `Tile`, meaning it represents one part of the grid that stores the information of the cave's geography
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Tile {
    Wall {
        breaks: bool,
        ore: u8,
    },
    Ground {
        concealed: bool,
    },

    /// Convenience Tile, Should never see be seen in actual grids... only exist for comparison
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

    pub fn new_hover_handler(
        tex_manager: &mut TextureManager,
        loader: &Loader,
        mut tex_storage: &mut AssetStorage<Texture>,
    ) -> HoverHandlerComponent {
        let hover_mat = tex_manager.get_handle_or_load("ground_hover", &loader, &mut tex_storage);

        let bounding_box = Cuboid::new(Vector3::new(0.5, 0.01, 0.5));
        Box::new(SimpleHoverHandler::new(bounding_box, hover_mat))
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

impl Clickable for Tile {
    fn on_click(&self, entity: Entity, world: &World) {
        if let Some(SelectedRockRaider(rock_raider)) = *world.write_resource() {
            // Destination is the clicked entity
            let level_grid = world.read_resource::<LevelGrid>();
            let tiles = world.read_storage::<Tile>();
            let transforms = world.read_storage::<Transform>();

            let transform = transforms.get(rock_raider).unwrap().translation();
            let x = (transform.x + 0.5) as i32;
            let y = (transform.z + 0.5) as i32;

            let start = level_grid.get(x, y).unwrap();
            let path = level_grid.find_path(start, entity, &tiles, &transforms);

            if let Some(path) = path {
                world.write_storage().insert(rock_raider, path).unwrap();
            }
        }
        *world.write_resource::<Option<SelectedRockRaider>>() = None;
    }

    fn new_click_handler(&self) -> ClickHandlerComponent {
        // TODO Refactor
        // This is working, because there are currently no different clickhandler for different Tiles.
        // A rr does not move to a Wall, because there will be no Path to the Wall(the destination is not `walkable()`)
        // is updated as soon as different ClickHandler are required
        Box::new(Tile::Any) as Box<dyn Clickable>
    }
}
