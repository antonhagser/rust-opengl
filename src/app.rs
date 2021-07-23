use std::sync::{Arc, Mutex};

use glutin::platform::run_return::EventLoopExtRunReturn;

use crate::{assets::AssetManager, context::Context, engine::Engine, system::System};

use self::builder::AppBuilder;

mod builder;

pub struct App<'a, T> where T: System {
    name: &'a str,
    system: Arc<Mutex<T>>,
    size: (u32, u32),

    context: Option<Context>
}

impl<'a, T> App<'a, T> where T: System {
    pub fn builder() -> AppBuilder<'a, T> {
        AppBuilder::default()
    }

    pub fn run(mut self) {
        let mut asset_manager = AssetManager::new();
        asset_manager.awake_hotreload("./assets".into());

        let (engine, mut event_loop) = Engine::new(self.name.to_string(), self.size.0, self.size.1, asset_manager.clone());
        let context = Context::new(engine, asset_manager);
        self.context = Some(context);

        // Awake engine
        self.context.as_mut().unwrap().engine_mut().awake();

        // Awake system
        self.system.lock().unwrap().awake(self.context.as_mut().unwrap());

        // Start event loop
        let start_time = std::time::Instant::now();
        event_loop.run_return(move |event, _, control_flow| {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(e) => {
                        unsafe {
                            gl::Viewport(
                                0,
                                0,
                                e.width as gl::types::GLint,
                                e.height as gl::types::GLint,
                            );
                        };
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    let renderer = self.context.as_mut().unwrap().engine_mut().renderer_mut();
                    
                    // Clear screen
                    renderer.clear();

                    #[cfg(debug_assertions)]
                    renderer.update_editor();

                    self.system.lock().unwrap().update(self.context.as_mut().unwrap());

                    let renderer = self.context.as_mut().unwrap().engine_mut().renderer_mut();
                    renderer.swap_buffers();

                    self.system.lock().unwrap().late_update(self.context.as_mut().unwrap());
                }
                _ => (),
            }
            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    let renderer = self.context.as_mut().unwrap().engine_mut().renderer_mut();
                    renderer.window().window().request_redraw();

                    let elapsed_time = std::time::Instant::now().duration_since(start_time);
                    let elapsed_time = elapsed_time.as_millis() as u64;

                    let wait_millis = match 1000 / 144 >= elapsed_time {
                        true => 1000 / 144 - elapsed_time,
                        false => 0,
                    };
                    let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                    *control_flow = ControlFlow::WaitUntil(new_inst);
                }
            }
        });
    }
}