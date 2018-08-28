use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Matrix4, Point3, Vector3};
use amethyst::core::transform::GlobalTransform;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, Material, MaterialDefaults, Mesh, MeshData, PngFormat, PosTex, Projection,
    Shape, SpriteRenderData, Texture, TextureHandle, VirtualKeyCode,
};

pub struct RockRaiders;

impl<'a, 'b> State<GameData<'a, 'b>> for RockRaiders {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "ground.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        //world.register::<Paddle>();

        initialise_ground(world, spritesheet);
        initialise_camera(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let initial_camera_position = Point3::new(0.0, 0.0, -15.0);
    let initial_point = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
        .with(GlobalTransform(Matrix4::look_at(
            initial_camera_position,
            initial_point,
            up,
        ))).build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_ground(world: &mut World, spritesheet: TextureHandle) {
    let entity = world
        .create_entity()
        .with(GlobalTransform::default())
        .build();

    let cube_mesh: MeshData = Shape::generate::<Vec<PosTex>>(&Shape::Cube, None);

    let mesh = {
        let loader = world.read_resource::<Loader>();
        let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
        loader.load_from_data(cube_mesh, (), &mesh_storage)
    };

    let material = {
        let default = world.read_resource::<MaterialDefaults>();
        Material {
            albedo: spritesheet,
            ..default.0.clone()
        }
    };

    let mut renderer = world.system_data::<SpriteRenderData>();
    renderer
        .meshes
        .insert(entity, mesh)
        .expect("cannot insert mesh");
    renderer
        .materials
        .insert(entity, material)
        .expect("cannot insert material");
}
