use amethyst::core::specs::prelude::{Read, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entity, SystemData, Resources};
use amethyst::renderer::{MeshHandle, TextureHandle};
use std::time::Duration;
use amethyst::shrev::{EventChannel, ReaderId};


// TODO REMOVE PUB IT A HACK
pub struct GroundRevealSystem {
    // TODO put entities in there
    pub reader: Option<ReaderId<i32>>
}

impl<'a> System<'a> for GroundRevealSystem {
    type SystemData = (
        Read<'a, Time>,
        Write<'a, EventChannel<i32>>,
        WriteStorage<'a, MeshHandle>,
        WriteStorage<'a, TextureHandle>,
        WriteStorage<'a, Transform>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<i32>>().register_reader());
    }

    fn run(
        &mut self,
        (time, mut channel, mut meshes, mut textures, mut transforms): Self::SystemData,
    ) {
        if let Some(reader) = self.reader.as_mut() {
            // this is actually broken, the meta readers length (dict of Channel containing all reader seems to be 0) ???? Dunno
            // to be continiued by @karyon
            for event in channel.read(reader) {
                //TODO
            }
        }
    }
}
