mod camera_movement;
mod clicking;
mod hovering;
mod mouse_ray;

pub use self::camera_movement::CameraMovementSystem;
pub use self::clicking::*;
pub use self::hovering::*;
pub use self::mouse_ray::*;

use amethyst::{
    core::{
        shrev::{EventChannel, ReaderId},
        ecs::{Read, Resources, SystemData},
        EventReader,
    },
    ui::UiEvent,
    winit::Event,
};

/// All Events that are handled in the States `handle_event`
#[derive(Clone, EventReader)]
#[reader(GameEventReader)]
pub enum GameEvent {
    /// Events sent by the winit window.
    Window(Event),
    /// Events sent by the ui system.
    Ui(UiEvent),
    /// Event sent by the hover system.
    Hover(HoverEvent),
}
