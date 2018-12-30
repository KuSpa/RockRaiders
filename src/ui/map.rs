use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Parent,
    ecs::{Entity, Read, ReadExpect, ReadStorage, System, World, Write, WriteStorage},
    prelude::Builder,
    renderer::{Texture, TextureData},
    ui::{Anchor, UiImage, UiTransform},
};

use entities::Tile;
use level::{LevelGrid, TileGrid};

pub struct MapUpdateSystem;

impl<'a> System<'a> for MapUpdateSystem {
    type SystemData = (
        Read<'a, LevelGrid>,
        ReadExpect<'a, Loader>,
        Read<'a, Option<UiMap>>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, UiImage>,
        Write<'a, AssetStorage<Texture>>,
    );

    fn run(
        &mut self,
        (level_grid,loader, map, tile_storage, mut ui_image_storage, mut tex_storage): Self::SystemData,
    ) {
        let map = (*map).as_ref().unwrap();
        for x in 0..level_grid.x_len() {
            for y in 0..level_grid.y_len() {
                let (x, y) = (x as i32, y as i32);
                let tile = level_grid.get_tile(x, y, &tile_storage).unwrap();
                map.update_color(
                    x,
                    y,
                    &tile,
                    &loader,
                    &mut ui_image_storage,
                    &mut tex_storage,
                );
            }
        }
    }
}

/// Green for Ground
const GROUND_COLOR: [f32; 4] = [0., 1., 0., 1.];
const WALL_COLOR: [f32; 4] = [0.5, 0.4, 0., 1.0];

/// Holds all entities of the UiMap. Those are either small images to represent the tiles or other small quads, that represent the RockRaider.
#[derive(Default)]
pub struct UiMap {
    /// Like the LevelGrid, this stores the entities of the MapGrid
    grid: Vec<Vec<Entity>>,
    /// A Vec of all RR on the Map
    rr: Vec<Entity>,
}

impl UiMap {
    /// Creates a new UiMap from a specification. Requires world access.
    pub fn from(mut grid: TileGrid, world: &mut World) -> Self {
        // x and y seem swizzled. They are, but this is intended, since the camera is rotated. Furthermore, y needs to be inverted, because the coordination system is inverted in the z direction (right handed)...
        let max_y = grid[0].len() as f32;
        let max_x = grid.len() as f32;
        let position = UiTransform::new(
            "UiMap".to_string(),
            Anchor::TopLeft,
            100.,
            -100.,
            1.,
            max_x * 10.,
            max_y * 10.,
            0,
        );
        let parent = world.create_entity().with(position).build();

        let grid: Vec<Vec<Entity>> = grid
            .iter_mut()
            .enumerate()
            .map(|(x, tile_vec)| {
                tile_vec
                    .iter_mut()
                    .enumerate()
                    .map(|(y, tile)| {
                        // TODO take this out of loop with `systemdata`...
                        let texture = {
                            let loader = world.read_resource::<Loader>();
                            let mut storage = world.write_resource::<AssetStorage<Texture>>();
                            UiImage {
                                texture: loader.load_from_data(
                                    TextureData::color(Self::tile_color(tile)),
                                    (),
                                    &storage,
                                ),
                            }
                        };

                        let position = UiTransform::new(
                            "MapTile".to_string(),
                            Anchor::TopLeft,
                            x as f32 * 10.,
                            max_y - y as f32 * 10.,
                            2.,
                            8.,
                            8.,
                            0,
                        );
                        world
                            .create_entity()
                            .with(position)
                            .with(texture)
                            .with(Parent { entity: parent })
                            .build()
                    })
                    .collect()
            })
            .collect();

        UiMap {
            grid,
            rr: Vec::default(),
        }
    }

    pub fn update_color(
        &self,
        x: i32,
        y: i32,
        tile: &Tile,
        loader: &Loader,
        ui_image_storage: &mut WriteStorage<UiImage>,
        tex_storage: &mut Write<AssetStorage<Texture>>,
    ) {
        // TODO #23 remove runtime warnings because of unnecessary loading
        let image = UiImage {
            texture: loader.load_from_data(
                TextureData::color(Self::tile_color(tile)),
                (),
                tex_storage,
            ),
        };
        let entity = self.grid[x as usize][y as usize];
        ui_image_storage.insert(entity, image).unwrap();
    }

    fn tile_color(tile: &Tile) -> [f32; 4] {
        match tile {
            Tile::Ground { concealed: false } => GROUND_COLOR,
            _ => WALL_COLOR,
        }
    }
}
