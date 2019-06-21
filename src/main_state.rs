use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
};
use eventhandling::GameEvent;
use GameScene;

use level::LevelState;

/// The main state of the Game. When started, this will be the first state on the stack.
pub struct MainState;

impl MainState {
    fn scene() -> GameScene {
        GameScene::Main
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, GameEvent> for MainState {
    fn on_start(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = MainState::scene();
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = MainState::scene();
    }

    fn handle_event(
        &mut self,
        _: StateData<GameData>,
        event: GameEvent,
    ) -> Trans<GameData<'a, 'b>, GameEvent> {
        if let GameEvent::Window(event) = &event {
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

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, GameEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}
