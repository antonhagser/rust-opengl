#![feature(get_mut_unchecked)]
#![feature(destructuring_assignment)]

use std::{cell::RefCell, rc::Rc};

use assets::AssetManager;
use renderer::Renderer;

pub mod assets;
pub mod color;
pub mod renderer;

#[macro_use]
extern crate log;

pub struct Engine<T> {
    data: T,
    renderer: Option<Renderer<'static, 2>>,
}

impl<T: 'static> Engine<T>
where
    T: EngineImplementor<T> + Default,
{
    pub fn new() -> Rc<RefCell<Self>> {
        let engine = Rc::new(RefCell::new(Self {
            data: T::default(),
            renderer: None,
        }));
        let mut eng = engine.clone();
        let mutable_engine = unsafe { Rc::get_mut_unchecked(&mut eng) };
        mutable_engine
            .borrow_mut()
            .data
            .awake(unsafe { engine.as_ptr().as_mut().unwrap() });

        engine
    }

    pub fn run(engine: Rc<RefCell<Self>>) {
        // Get mutable reference to engine
        // Used to trigger events on client code
        let mut eng = engine.clone();
        let mutable_engine = unsafe { Rc::get_mut_unchecked(&mut eng) };

        // Create a new glutin event code
        let event_loop = glutin::event_loop::EventLoop::new();

        // Glutin window initialization
        let window = glutin::window::WindowBuilder::new()
            .with_title(mutable_engine.get_mut().data.name())
            .with_inner_size(glutin::dpi::LogicalSize::new(1280, 720));

        // Get OpenGL context
        let gl_window = glutin::ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap();

        // Make window current
        let gl_window = unsafe { gl_window.make_current().unwrap() };

        // Initialize renderer
        let mut mut_engine = mutable_engine.borrow_mut();
        mut_engine.renderer = Some(Renderer::<2>::new(gl_window));
        let mut asset_manager = AssetManager::new();
        asset_manager.awake_hotreload("./assets/".into());

        // Register asset manager
        mut_engine
            .renderer
            .as_mut()
            .unwrap()
            .register_asset_manager(asset_manager);

        // Trigger awake function and load opengl
        mut_engine.renderer.as_mut().unwrap().awake();

        // Start time
        let start_time = std::time::Instant::now();

        // Trigger start event
        mut_engine
            .data
            .start(unsafe { engine.as_ptr().as_mut().unwrap() });

        // Drop the current mutable engine
        drop(mut_engine);

        // Run window event loop
        info!("Starting window event loop");
        event_loop.run(move |event, _, control_flow| {
            let mut eng = engine.clone();
            let mut_engine = unsafe { Rc::get_mut_unchecked(&mut eng) };
            let mut mut_engine = mut_engine.borrow_mut();

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
                    // Clear screen
                    mut_engine.renderer.as_mut().unwrap().clear();

                    #[cfg(debug_assertions)]
                    mut_engine.renderer.as_mut().unwrap().update_editor();

                    mut_engine
                        .data
                        .update(unsafe { engine.as_ptr().as_mut().unwrap() });

                    mut_engine.renderer.as_mut().unwrap().swap_buffers();

                    mut_engine
                        .data
                        .late_update(unsafe { engine.as_ptr().as_mut().unwrap() });
                }
                _ => (),
            }
            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    mut_engine
                        .renderer
                        .as_mut()
                        .unwrap()
                        .window()
                        .window()
                        .request_redraw();
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

    pub fn enable_logger(&mut self, env: &str) {
        std::env::set_var("RUST_LOG", env);
        env_logger::init();
        trace!("Enabled engine logger with env: {:?}.", env);
    }

    /// Get a mutable reference to the engine's renderer.
    pub fn renderer(&mut self) -> &mut Renderer<'static, 2> {
        self.renderer.as_mut().unwrap()
    }
}

unsafe impl<T> Send for Engine<T> {}
unsafe impl<T> Sync for Engine<T> {}

pub trait EngineImplementor<T>
where
    T: EngineImplementor<T>,
{
    fn awake(&mut self, engine: &mut Engine<T>);
    fn start(&mut self, engine: &mut Engine<T>);
    fn update(&mut self, engine: &mut Engine<T>);
    fn late_update(&mut self, engine: &mut Engine<T>);
    fn name(&self) -> &'static str;
}

#[no_mangle]
pub extern "system" fn opengl_error_handling(
    source: gl::types::GLenum,
    kind: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _: *mut gl::types::GLvoid,
) {
    use colored::Colorize;

    let msg = unsafe { std::ffi::CStr::from_ptr(message) };

    warn!(
        "{}{} {:#X} {} {:#X} {} {:#X} {} {:#X} {} {}",
        "OpenGL Error:\n\t",
        "Source:".green(),
        source,
        "Kind:".green(),
        kind,
        "Id:".green(),
        id,
        "Severity:".green(),
        severity,
        "\n\tMessage:".green(),
        msg.to_string_lossy().red()
    );
}
