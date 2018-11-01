use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection, VirtualKeyCode, WindowMessages};
use game_data::CustomGameData;

use level::Level;

pub struct RockRaiders;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for RockRaiders {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;
        //initialize_cursor(world);

        initialize_camera(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                return Trans::Push(Box::new(Level));
            }
        }
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
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

fn initialize_cursor(world: &mut World) {
    use amethyst::renderer::mouse::grab_cursor;

    //TODO - custom cursor icon xD

    let mut msg = world.write_resource::<WindowMessages>();
    grab_cursor(&mut msg);
}
