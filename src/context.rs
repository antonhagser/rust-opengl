use crate::{assets::AssetManager, engine::Engine};

pub struct Context {
    engine: Engine,
    asset_manager: AssetManager,
}

impl Context {
    pub fn new(engine: Engine, asset_manager: AssetManager) -> Self {
        Context { engine, asset_manager }
    }

    /// Get a reference to the context's engine.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Get a mutable reference to the context's engine.
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}