use amethyst::assets::Loader;
use amethyst::core::transform::components::Parent;
use amethyst::ecs::prelude::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{PngFormat, TextureMetadata};
use amethyst::ui::*;

pub struct OxygenBar {
    pub parent: Entity,
    pub left_end: Entity,
    pub right_end: Entity,
    pub background_bar: Entity,
    pub filled_bar: Entity,
}

impl OxygenBar {
    pub fn add_to_world(world: &mut World) {
        let transform = UiTransform::new(
            "left_end".to_string(),
            Anchor::BottomLeft,
            100.,
            100.,
            0.,
            100.,
            11.,
            0,
        );

        let parent = world.create_entity().with(transform).build();

        let left_end = Self::left_end(&parent, world);
        let right_end = Self::right_end(&parent, world);
        let background_bar = Self::empty_bar(&parent, world);
        let filled_bar = Self::filled_bar(&background_bar, world);

        world.add_resource(OxygenBar {
            parent,
            left_end,
            right_end,
            background_bar,
            filled_bar,
        });
    }

    fn left_end(parent: &Entity, world: &mut World) -> Entity {
        let transform = UiTransform::new(
            "left_end".to_string(),
            Anchor::MiddleLeft,
            0.,
            0.,
            2., //Maybe add offset
            8.,
            11.,
            0,
        );

        let texture = world.read_resource::<Loader>().load(
            "ui/oxygen_bar/left_end.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(transform)
            .with(UiImage { texture })
            .with(Parent { entity: *parent })
            .build()
    }

    fn right_end(parent: &Entity, world: &mut World) -> Entity {
        let transform = UiTransform::new(
            "right_end".to_string(),
            Anchor::MiddleRight,
            0.,
            0.,
            2.,
            8.,
            11.,
            0,
        );

        let texture = world.read_resource::<Loader>().load(
            "ui/oxygen_bar/right_end.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(transform)
            .with(UiImage { texture })
            .with(Parent { entity: *parent })
            .build()
    }

    fn empty_bar(parent: &Entity, world: &mut World) -> Entity {
        let transform = UiTransform::new(
            "empty_bar".to_string(),
            Anchor::MiddleLeft,
            50.,
            0.,
            0.,
            100.,
            7.,
            0,
        );

        let texture = world.read_resource::<Loader>().load(
            "ui/oxygen_bar/empty_bar.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(transform)
            .with(Parent { entity: *parent })
            .with(UiImage { texture })
            .build()
    }

    fn filled_bar(parent: &Entity, world: &mut World) -> Entity {
        let transform = UiTransform::new(
            "left_end".to_string(),
            Anchor::MiddleLeft,
            50.,
            0.,
            1.,
            100.,
            7.,
            0,
        );

        let texture = world.read_resource::<Loader>().load(
            "ui/oxygen_bar/filled_bar.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(transform)
            .with(UiImage { texture })
            .with(Parent { entity: *parent })
            .build()
    }
}
