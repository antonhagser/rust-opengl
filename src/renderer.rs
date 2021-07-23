use std::collections::HashMap;

use glutin::{window::Window, ContextWrapper, PossiblyCurrent};

use crate::{assets::AssetManager, color::prelude::*};
use pipeline_info::PipelineInfo;

use self::shader::ShaderProgram;

type GLWindow = ContextWrapper<PossiblyCurrent, Window>;

pub mod buffer;
pub mod camera;
pub mod pipeline_info;
pub mod render_target;
pub mod shader;
pub mod texture;

pub struct Renderer<'a, const T: usize> {
    window: GLWindow,
    plinfo: Option<PipelineInfo<'a>>,
    clear_color: RGBAColor<f32>,

    // Transfer ownership of data to engine-manager
    asset_manager: AssetManager,
    shader_programs: HashMap<String, ShaderProgram<'a>>,
}

impl<'a, const T: usize> Renderer<'a, T> {
    pub fn new(window: GLWindow, asset_manager: AssetManager) -> Self {
        Renderer {
            window,
            plinfo: None,
            clear_color: (HexColor::<u8>::new(0x131519).rgba() / 255),

            asset_manager,
            shader_programs: HashMap::new(),
        }
    }

    // Awake triggers loading of OpenGL
    pub fn awake(&mut self) {
        // Load OpenGL function pointers
        trace!("Loading OpenGL proc addresses");
        gl::load_with(|symbol| self.window().get_proc_address(symbol));
        trace!("Finished loading OpenGL proc addresses");

        // Enable OpenGL debug logging
        unsafe {
            gl::DebugMessageCallback(
                Some(opengl_error_handling),
                0 as *const gl::types::GLvoid,
            );
        }

        // Request pipeline information
        self.plinfo = Some(PipelineInfo::new(&self.window()));

        // Set clear color
        self.set_clear_color(self.clear_color);

        // Print OpenGL version for debugging
        trace!("OpenGL system information:");
        trace!("\tOpenGl Version: {}", self.plinfo().version());
        trace!("\tGraphics Vendor: {}", self.plinfo().vendor());
        trace!("\tGraphics Renderer: {}", self.plinfo().renderer());

        // Print important information about driver
        trace!("OpenGL Driver Stats:");
        trace!(
            "\tMax Vertex Attrib: {}",
            self.plinfo().max_vertex_attribs()
        );

        info!("Finished activating renderer");
    }

    /// Trigger clear
    pub fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Swap buffers
    pub fn swap_buffers(&self) {
        self.window()
            .swap_buffers()
            .expect("Failed to swap buffers");
    }

    #[cfg(debug_assertions)]
    pub fn update_editor(&mut self) {
        let channel = self.asset_manager
            .channel()
            .as_ref()
            .expect("No reload channel has been launched");

        let msg = channel.try_recv();
        match msg {
            Ok(asset) => {
                trace!("Received reload editor event");
                match asset.1 {
                    crate::assets::AssetKind::Shader => {
                        let id = asset.0;

                        // Fetc the asset from the asset manager
                        let asset = self.asset_manager.asset(&id).unwrap();

                        // In the case of a shader, the identifier is used to identify the shaderprogram to reload
                        let program = self.shader_programs.get_mut(asset.identifier()).unwrap();

                        // Reload asset inside shader program
                        program.reload(asset.value());
                    }
                    crate::assets::AssetKind::Texture => {}
                    crate::assets::AssetKind::Video => {}
                };
            }
            Err(_) => {}
        }
    }

    /// Get a reference to the renderer's window.
    pub fn window(&self) -> &ContextWrapper<PossiblyCurrent, Window> {
        &self.window
    }

    /// Get a reference to the renderer's plinfo.
    pub fn plinfo(&self) -> &PipelineInfo<'a> {
        self.plinfo.as_ref().unwrap()
    }

    /// Set the renderer's clear color.
    pub fn set_clear_color(&mut self, clear_color: RGBAColor<f32>) {
        trace!("Set clear color buffer bit");
        self.clear_color = clear_color;
        unsafe {
            gl::ClearColor(
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            );
        }
    }

    /// Get a reference to the renderer's clear color.
    pub fn clear_color(&self) -> &RGBAColor<f32> {
        &self.clear_color
    }

    /// Get a mutable reference to the renderer's shader programs.
    pub fn shader_programs(&mut self) -> &mut HashMap<String, ShaderProgram<'a>> {
        &mut self.shader_programs
    }
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