use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Entity, Read, ReadStorage, System, Write, WriteStorage},
    ui::UiTransform,
};
use entities::RockRaider;
use util::{amount_in, find_ui_by_name};

pub struct OxygenSystem;

impl<'a> System<'a> for OxygenSystem {
    type SystemData = (
        Write<'a, Option<OxygenBar>>,
        Read<'a, Time>,
        Write<'a, Option<Oxygen>>,
        ReadStorage<'a, RockRaider>,
        WriteStorage<'a, UiTransform>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut ui, time, mut oxygen, rock_raiders, mut ui_transforms, entities): Self::SystemData,
    ) {
        if let Some(ref mut oxygen) = *oxygen {
            let breathed_oxygen = amount_in(&rock_raiders) as f32 * time.delta_seconds();
            oxygen.remaining_oxygen -= breathed_oxygen;

            if oxygen.remaining_oxygen <= 0. {
                panic!("No oxygen left for Breathing. You lost");
            }

            if let Some(ui) = &*ui {
                let percentage = oxygen.remaining_oxygen / oxygen.max_oxygen;

                let max_width = ui_transforms.get(ui.empty_bar.clone()).unwrap().width;
                let mut transform = ui_transforms.get_mut(ui.filled_bar).unwrap();
                transform.width = max_width * percentage;
                transform.local_x = max_width * percentage / 2.;
                return;
            }

            let empty_bar = find_ui_by_name("empty_bar", &entities, &ui_transforms);
            let filled_bar = find_ui_by_name("filled_bar", &entities, &ui_transforms);
            if empty_bar.is_some() && filled_bar.is_some() {
                *ui = Some(OxygenBar {
                    filled_bar: filled_bar.unwrap(),
                    empty_bar: empty_bar.unwrap(),
                })
            }
        } else {
            error!("Running oxygen_system with invalid data");
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

/// This is the part of the ui that represents the filled oxygen-O-meter
///
/// The entity's length represents the amount of available oxygen. This will (compared to the `empty_bar`, which has a static length representing `max_oxygen`) show the percentage of available oxygen in the cave.
///
pub struct OxygenBar {
    pub filled_bar: Entity,
    pub empty_bar: Entity,
}
