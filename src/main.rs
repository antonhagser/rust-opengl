#![allow(dead_code)]

use assets::{Asset, AssetKind, AssetManager};
use renderer::Renderer;

pub mod assets;
pub mod color;
pub mod renderer;

#[macro_use]
extern crate log;

fn main() {
    // Initialize logger
    std::env::set_var("RUST_LOG", "engine");
    env_logger::init();
    info!("Loading OpenGL graphics engine");

    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title("OpenGL")
        .with_inner_size(glutin::dpi::LogicalSize::new(1280, 720));

    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    let gl_window = unsafe { gl_window.make_current().unwrap() };

    // Initialize renderer
    let mut renderer = Renderer::<2>::new(gl_window);
    let asset_manager = AssetManager::new();

    // Trigger awake function and load opengl
    renderer.awake();
    renderer.register_asset_manager(asset_manager);
    let manager = renderer.asset_manager().as_mut().unwrap();
    manager.awake_hotreload("./assets/".into());
    
    let pathbuf = std::path::Path::new("./assets/shaders/default.vert").to_path_buf();
    let asset = Asset::new(pathbuf.clone(), AssetKind::Text).unwrap();
    manager.create_asset(asset);
    manager.register_for_hotreload(pathbuf);

    // Start time
    let start_time = std::time::Instant::now();
    let mut last_delta: f32 = 0.0;

    // Run window event loop
    info!("Starting window event loop");
    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    renderer.pos = (position.x, position.y);
                }
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
                renderer.draw(last_delta);
                renderer.swap_buffers();
            }
            _ => (),
        }
        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                renderer.window().window().request_redraw();
                let elapsed_time = std::time::Instant::now().duration_since(start_time);
                last_delta = elapsed_time.as_secs_f32();
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
