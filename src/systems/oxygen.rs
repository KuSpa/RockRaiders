use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read,ReadExpect, ReadStorage, System, Write, WriteStorage},
    ui::UiTransform,
};
use entities::RockRaider;
use ui::OxygenBar;
use util::amount_in;

pub struct OxygenSystem;

impl<'a> System<'a> for OxygenSystem {
    type SystemData = (
        Option<ReadExpect<'a, OxygenBar>>,
        Read<'a, Time>,
        Write<'a, Oxygen>,
        ReadStorage<'a, RockRaider>,
        WriteStorage<'a, UiTransform>,
    );

    fn run(&mut self, (ui, time, mut oxygen, rock_raiders, mut ui_transforms): Self::SystemData) {
        let breathed_oxygen = amount_in(&rock_raiders) as f32 * time.delta_seconds();
        oxygen.remaining_oxygen -= breathed_oxygen;

        if oxygen.remaining_oxygen <= 0. {
            panic!("No oxygen left for Breathing. You lost");
        }

        if let Some(ui) = ui {
            let percentage = oxygen.remaining_oxygen / oxygen.max_oxygen;

            let max_width = ui_transforms.get(ui.background_bar.clone()).unwrap().width;
            let mut transform = ui_transforms.get_mut(ui.filled_bar.clone()).unwrap();
            transform.width = max_width * percentage;
            transform.local_x = max_width * percentage / 2.;
        }
    }
}

/// Wrapper around the amount of oxygen left in the cave.
#[derive(Default)]
pub struct Oxygen {
    pub max_oxygen: f32,
    pub remaining_oxygen: f32,
}

impl Oxygen {
    pub fn new(amount: f32) -> Self {
        Oxygen {
            max_oxygen: amount,
            remaining_oxygen: amount,
        }
    }
}
