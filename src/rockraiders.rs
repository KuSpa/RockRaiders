use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Deg, Matrix4, Point3, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, Material, MaterialDefaults, Mesh, MeshData, PngFormat, PosTex, Projection,
    Shape, SpriteRenderData, Texture, TextureHandle, VirtualKeyCode, WindowMessages,
};

pub struct RockRaiders;

impl<'a, 'b> State<GameData<'a, 'b>> for RockRaiders {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialize_cursor(world);

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

        initialize_ground(world, spritesheet.clone());
        initialize_camera(world);
        initialize_object(world, spritesheet)
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

/// initialize the camera.
fn initialize_camera(world: &mut World) {
    let mut mat = Transform::default();
    mat.move_global(Vector3::new(-2.0, 5.0, 2.0));
    mat.yaw_global(Deg(-45.0));
    mat.pitch_local(Deg(-45.0));

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
        .with(mat)
        .with(GlobalTransform::default())
        .build();
}

fn initialize_ground(world: &mut World, texture: TextureHandle) {
    let spritesheet = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "groundy.png",
            PngFormat,
            Default::default(),
            (),
            &texture_storage,
        )
    };

    let entity = world
        .create_entity()
        .with(GlobalTransform(Matrix4::from_scale(5.0)))
        .build();

    let ground_mesh = gen_rectangle_mesh();

    let mesh = {
        let loader = world.read_resource::<Loader>();
        let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
        loader.load_from_data(ground_mesh.into(), (), &mesh_storage)
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

fn initialize_object(world: &mut World, spritesheet: TextureHandle) {
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

fn initialize_cursor(world: &mut World) {
    use amethyst::renderer::mouse::grab_cursor;

    //TODO - custom cursor icon xD

    let mut msg = world.write_resource::<WindowMessages>();
    grab_cursor(&mut msg);
}

fn gen_rectangle_mesh() -> Vec<PosTex> {
    vec![
        PosTex {
            position: [0., 0., 0.],
            tex_coord: [0., 0.],
        },
        PosTex {
            position: [1., 0., 0.],
            tex_coord: [1., 0.],
        },
        PosTex {
            position: [1., 0., -1.],
            tex_coord: [1., 1.],
        },
        PosTex {
            position: [1., 0., -1.],
            tex_coord: [1., 1.],
        },
        PosTex {
            position: [0., 0., -1.],
            tex_coord: [0., 1.],
        },
        PosTex {
            position: [0., 0., 0.],
            tex_coord: [0., 0.],
        },
    ]
}
