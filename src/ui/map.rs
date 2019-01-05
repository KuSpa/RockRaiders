use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Parent, Transform},
    ecs::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, World, Write, WriteStorage,
    },
    prelude::Builder,
    renderer::Texture,
    ui::{Anchor, UiImage, UiTransform},
};

use assetmanagement::TextureManager;
use entities::{RockRaider, Tile};
use level::{LevelGrid, TileGrid};
use util::find_ui_by_name;

/// Green for Ground
const GROUND_COLOR: &'static str = "GREEN";
const WALL_COLOR: &'static str = "BEIGE";
const RR_COLOR: &'static str = "RED";

pub struct UiRockRaiderSystem;

impl<'a> System<'a> for UiRockRaiderSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Loader>,
        Write<'a, TextureManager>,
        Write<'a, Option<UiMap>>,
        WriteStorage<'a, UiImage>,
        WriteStorage<'a, UiTransform>,
        ReadStorage<'a, Transform>,
        Write<'a, AssetStorage<Texture>>,
        ReadStorage<'a, RockRaider>,
        WriteStorage<'a, Parent>,
    );

    fn run(
        &mut self,
        (
            entities,
            loader,
            mut texture_manager,
            mut map,
            mut ui_image_storage,
            mut ui_transform_storage,
            transform_storage,
            mut tex_storage,
            rock_raider_storage,
            mut parent_storage,
        ): Self::SystemData,
    ) {
        let map = (*map).as_mut().unwrap();
        // delete all ui_rr
        for entity in map.rr.drain(..) {
            entities.delete(entity).unwrap();
        }
        // add all ui_rr
        for (_, transform) in (&rock_raider_storage, &transform_storage).join() {
            // create a little quad in the map
            let image = UiImage {
                texture: texture_manager.get_handle_or_load(RR_COLOR, &loader, &mut tex_storage),
            };
            let position = UiTransform::new(
                "UiMapRR".to_string(),
                Anchor::TopLeft,
                transform.translation().x * 20.,
                -transform.translation().z * 20. + 6.,
                3.,
                12.,
                12.,
                0,
            );
            let parent = find_ui_by_name("UiMap", &entities, &ui_transform_storage).unwrap();
            map.rr.push(
                entities
                    .build_entity()
                    .with(Parent { entity: parent }, &mut parent_storage)
                    .with(position, &mut ui_transform_storage)
                    .with(image, &mut ui_image_storage)
                    .build(),
            );
        }
    }
}

pub struct UiMapUpdateSystem;

impl<'a> System<'a> for UiMapUpdateSystem {
    type SystemData = (
        Read<'a, LevelGrid>,
        ReadExpect<'a, Loader>,
        Write<'a, TextureManager>,
        Read<'a, Option<UiMap>>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, UiImage>,
        Write<'a, AssetStorage<Texture>>,
    );

    fn run(
        &mut self,
        (
            level_grid,
            loader,
            mut texture_manager,
            map,
            tile_storage,
            mut ui_image_storage,
            mut tex_storage,
        ): Self::SystemData,
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
                    &mut texture_manager,
                    &mut ui_image_storage,
                    &mut tex_storage,
                );
            }
        }
    }
}

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
            200.,
            -200.,
            1.,
            max_x * 20.,
            max_y * 20.,
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
                        let texture = {
                            // We COULD move this out of the loop. but we don't want to. see the comment above `create_entity`
                            let loader = world.read_resource::<Loader>();
                            let mut texture_storage =
                                world.write_resource::<AssetStorage<Texture>>();
                            let mut texture_manager = world.write_resource::<TextureManager>();
                            UiImage {
                                texture: texture_manager.get_handle_or_load(
                                    Self::tile_color(tile),
                                    &loader,
                                    &mut texture_storage,
                                ),
                            }
                        };

                        let position = UiTransform::new(
                            "MapTile".to_string(),
                            Anchor::TopLeft,
                            x as f32 * 20.,
                            max_y - y as f32 * 20.,
                            2.,
                            18.,
                            18.,
                            0,
                        );

                        // This method requires mutability over the world and thus prevents any other references to the world.
                        // It's a convenience method to not have active storage access from elsewhere( see above for examples^^), while creating an entity and inserting into the borrowed storages.
                        // We may change that aas soon as we have performance issues.
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
        texture_manager: &mut Write<TextureManager>,
        ui_image_storage: &mut WriteStorage<UiImage>,
        tex_storage: &mut Write<AssetStorage<Texture>>,
    ) {
        let image = UiImage {
            texture: texture_manager.get_handle_or_load(
                Self::tile_color(tile),
                &loader,
                &tex_storage,
            ),
        };
        let entity = self.grid[x as usize][y as usize];
        ui_image_storage.insert(entity, image).unwrap();
    }

    fn tile_color(tile: &Tile) -> &'static str {
        match tile {
            Tile::Ground { concealed: false } => GROUND_COLOR,
            _ => WALL_COLOR,
        }
    }
}
