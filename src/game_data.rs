use amethyst::{
    config::Config,
    core::{ArcThreadPool, SystemBundle},
    ecs::prelude::{Dispatcher, DispatcherBuilder, System, World},
    renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex, RenderBundle, Stage},
    ui::DrawUi,
    DataInit, Error, Result,
};

use std::path::Path;

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    level_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    /// Update game data
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.level_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub level: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            level: DispatcherBuilder::new(),
        }
    }

    pub fn with_custom_renderer<A>(self, path: A) -> Result<Self>
    where
        A: AsRef<Path>,
    {
        let config = DisplayConfig::load(path);
        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawShaded::<PosNormTex>::new())
                .with_pass(DrawUi::new()),
        );
        self.with_core_bundle(RenderBundle::new(pipe, Some(config)))
    }

    pub fn with_core_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_running_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.level)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.level.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut level_dispatcher = self.level.with_pool(pool.clone()).build();
        core_dispatcher.setup(&mut world.res);
        level_dispatcher.setup(&mut world.res);

        CustomGameData {
            core_dispatcher,
            level_dispatcher,
        }
    }
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}
