use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, ReadStorage, System, Write},
};
use entities::RockRaider;
use util::amount_in;

pub struct OxygenSystem;

impl<'a> System<'a> for OxygenSystem {
    type SystemData = (
        Read<'a, Time>,
        Write<'a, Oxygen>,
        ReadStorage<'a, RockRaider>,
    );

    fn run(&mut self, (time, mut oxygen, rock_raiders): Self::SystemData) {
        let breathed_oxygen = amount_in(&rock_raiders) as f32 * time.delta_seconds();
        **oxygen -= breathed_oxygen;

        if **oxygen <= 0. {
            panic!("No oxygen left for Breathing. You lost");
        }
    }
}

/// Wrapper around the amount of oxygen left in the cave.
#[derive(Default)]
pub struct Oxygen {
    pub remaining_oxygen: f32,
}

use std::ops::{Deref, DerefMut};
impl Deref for Oxygen {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.remaining_oxygen
    }
}

impl DerefMut for Oxygen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.remaining_oxygen
    }
}
