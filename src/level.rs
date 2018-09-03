use tile::Grid;

pub struct Level;

impl<'a, 'b> State<GameData<'a, 'b>> for Level {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialize_camera(world);

        initialize_Grid(world);
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

initialize_grid(&mut world: World) {
    
}

/// initialize the camera.
fn initialize_camera(world: &mut World) {
    let mut mat = Transform::default();
    mat.move_global(Vector3::new(0., 3.0, 0.0));
    mat.yaw_global(Deg(-45.0));
    mat.pitch_local(Deg(-45.0));

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(1.0, Deg(60.0))))
        .with(mat)
        .with(GlobalTransform::default())
        .build();
}
