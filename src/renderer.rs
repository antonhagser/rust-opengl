use std::{collections::HashMap, sync::RwLock};

use glutin::{window::Window, ContextWrapper, PossiblyCurrent};
use render_target::{vertex_array::DefaultVertex, RenderTarget};
use shader::{FragmentShader, ShaderKind, VertexShader};

use crate::{assets::AssetManager, color::prelude::*};
use pipeline_info::PipelineInfo;

use self::{
    global::Global,
    shader::{ShaderProgram, Uniform},
};

type GLWindow = ContextWrapper<PossiblyCurrent, Window>;

pub mod buffer;
pub mod camera;
pub mod global;
pub mod pipeline_info;
pub mod render_target;
pub mod shader;

pub struct Renderer<'a, const T: usize> {
    window: GLWindow,
    plinfo: Option<PipelineInfo<'a>>,
    global: RwLock<Global>,
    clear_color: RGBAColor<f32>,

    // Transfer ownership of data to engine-manager
    asset_manager: Option<AssetManager>,
    shader_programs: HashMap<&'a str, ShaderProgram<'a>>,

    debug: Option<RenderTarget<'a, DefaultVertex, 4, 6>>,

    pub(crate) pos: (f64, f64),
}

impl<'a, const T: usize> Renderer<'a, T> {
    pub fn new(window: GLWindow) -> Self {
        Renderer {
            window,
            plinfo: None,
            clear_color: (HexColor::<u8>::new(0x131519).rgba() / 255),
            global: Global::new(),

            asset_manager: None,
            shader_programs: HashMap::new(),

            debug: None,

            pos: (0., 0.),
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
                Some(super::opengl_error_handling),
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

        let vertices = [
            DefaultVertex::new((-1.0, -1.0, 0.0), (1.0, 0.6, 0.3)),
            DefaultVertex::new((1.0, -1.0, 0.0), (1.0, 0.6, 0.3)),
            DefaultVertex::new((1.0, 1.0, 0.0), (1.0, 0.6, 0.3)),
            DefaultVertex::new((-1.0, 1.0, 0.0), (1.0, 0.6, 0.3)),
        ];
        self.debug = Some(RenderTarget::new(vertices, [0, 1, 2, 2, 3, 0]));

        // Load default shader
        info!("Loading default shader");
        let raw = include_str!("../assets/shaders/default.frag").to_string();
        let raw = std::ffi::CString::new(raw).expect("Failed at conveting shader to CString");
        let default_fragment =
            FragmentShader::from_source(raw.as_c_str()).expect("Failed to compile fragment");
        let raw = include_str!("../assets/shaders/default.vert").to_string();
        let raw = std::ffi::CString::new(raw).expect("Failed at conveting shader to CString");
        let default_vertex =
            VertexShader::from_source(raw.as_c_str()).expect("Failed to compile vertex");
        let default_program = ShaderProgram::new::<DefaultVertex>(vec![
            Box::new(default_fragment),
            Box::new(default_vertex),
        ])
        .expect("Failed to set-up shader program");

        // Assign default shader
        self.shader_programs.insert("default", default_program);

        // Register manager
        let manager = self
            .asset_manager()
            .as_mut()
            .expect("No asset-manager is registered");

        let pathbuf = std::path::Path::new("./assets/shaders/default.frag").to_path_buf();
        let asset = super::assets::Asset::new(
            "default.frag".into(),
            pathbuf.clone(),
            super::assets::AssetKind::Shader,
            "default",
            ShaderKind::to_u8(&ShaderKind::FragmentShader)
        )
        .expect("Failed to initialize asset");
        manager.create_asset(asset);
        manager.register_for_hotreload(pathbuf);

        info!("Finished activating renderer");
    }

    // Trigger draw
    pub fn draw(&mut self, delta_time: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let window_inner_size = self.window().window().inner_size();
        let def = self.shader_programs.get_mut("default").unwrap();

        def.bind();

        def.uniform2uiv(
            "u_Resolution",
            1,
            [window_inner_size.width, window_inner_size.height],
        )
        .expect("Failed assigning uniform");
        def.uniform2fv("u_Mouse", 1, [self.pos.0 as f32, self.pos.1 as f32])
            .expect("Failed assigning uniform");
        def.uniform1f("u_DeltaTime", delta_time)
            .expect("failed to assign deltatime");
        def.uniform1f(
            "u_Time",
            self.global
                .read()
                .unwrap()
                .start_time()
                .elapsed()
                .as_secs_f32(),
        )
        .expect("failed to assign deltatime");
        self.debug.as_ref().unwrap().draw();
    }

    // Swap buffers
    pub fn swap_buffers(&self) {
        self.window()
            .swap_buffers()
            .expect("Failed to swap buffers");
    }

    #[cfg(debug_assertions)]
    pub fn update_editor(&mut self) {
        let manager = self
            .asset_manager()
            .as_mut()
            .expect("No asset-manager is registered");
        let channel = manager
            .channel()
            .as_ref()
            .expect("No reload channel has been launched");

        let msg = channel.try_recv();
        match msg {
            Ok(m) => {
                trace!("Received reload editor event");
                match m.as_ref().read().unwrap().kind() {
                    crate::assets::AssetKind::Shader => {
                        // In the case of a shader, the identifier is used to identify the shaderprogram to reload
                        let program = self
                            .shader_programs
                            .get_mut(m.as_ref().read().unwrap().identifier().as_str())
                            .unwrap();
                        program.reload(m.clone());
                    }
                    crate::assets::AssetKind::Texture => {}
                    crate::assets::AssetKind::Video => {}
                };
            }
            Err(_) => {}
        }
    }

    /// Register a new asset manager
    pub fn register_asset_manager(&mut self, asset_manager: AssetManager) {
        self.asset_manager = Some(asset_manager);
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

    /// Get a reference to the renderer's asset manager.
    pub fn asset_manager(&mut self) -> &mut Option<AssetManager> {
        &mut self.asset_manager
    }
}
