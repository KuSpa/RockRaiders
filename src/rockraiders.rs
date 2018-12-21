use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
};
use game_data::CustomGameData;

use level::LevelState;

/// The main state of the Game. When started, this will be the first state on the stack.
pub struct MainState;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MainState {
    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                return Trans::Push(Box::new(LevelState {
                    mouse_button_was_down: false,
                }));
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
