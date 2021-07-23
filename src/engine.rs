use glutin::event_loop::EventLoop;

use crate::{assets::AssetManager, renderer::Renderer};

pub struct Engine {
    renderer: Renderer<'static, 2>,
}

impl Engine {
    pub fn new(name: String, width: u32, height: u32, asset_manager: AssetManager) -> (Engine, EventLoop<()>) {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let gl_window = glutin::ContextBuilder::new()
                    .build_windowed(window, &event_loop)
                    .unwrap();

        let gl_window = unsafe { gl_window.make_current().expect("failed to make window current") };

        (
            Engine {
                renderer: Renderer::new(gl_window, asset_manager),
            }, 
            event_loop
        )
    }

    pub fn awake(&mut self) {
        self.renderer.awake();
    }

    /// Get a mutable reference to the engine's renderer.
    pub fn renderer_mut(&mut self) -> &mut Renderer<'static, 2> {
        &mut self.renderer
    }

    /// Get a reference to the engine's renderer.
    pub fn renderer(&self) -> &Renderer<'static, 2> {
        &self.renderer
    }
}