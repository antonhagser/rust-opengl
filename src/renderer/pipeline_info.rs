use std::ffi::CStr;

use super::GLWindow;

#[derive(Debug, Clone)]
pub struct PipelineInfo<'a> {
    version: Option<Box<&'a str>>,
    vendor: Option<Box<&'a str>>,
    renderer: Option<Box<&'a str>>,
    max_vertex_attribs: gl::types::GLint,
}

impl<'a> PipelineInfo<'a> {
    pub fn new(_: &GLWindow) -> PipelineInfo<'a> {
        let mut plinfo = PipelineInfo {
            version: None,
            vendor: None,
            renderer: None,
            max_vertex_attribs: 0
        };

        plinfo.load_version();
        plinfo.load_vendor();
        plinfo.load_driver_stats();

        plinfo
    }

    /// Requests driver version information from OpenGL
    pub fn load_version(&mut self) {
        let version = unsafe { gl::GetString(gl::VERSION) };
        let version: &CStr = unsafe { CStr::from_ptr(version as *const i8) };
        self.version = Some(Box::new(version.to_str().unwrap()));
    }

    /// Requests vendor and renderer information from OpenGL
    pub fn load_vendor(&mut self) {
        // Vendor
        let vendor = unsafe { gl::GetString(gl::VENDOR) };
        let vendor: &CStr = unsafe { CStr::from_ptr(vendor as *const i8) };
        self.vendor = Some(Box::new(vendor.to_str().unwrap()));

        // Renderer
        let renderer = unsafe { gl::GetString(gl::RENDERER) };
        let renderer: &CStr = unsafe { CStr::from_ptr(renderer as *const i8) };
        self.renderer = Some(Box::new(renderer.to_str().unwrap()));
    }

    pub fn load_driver_stats(&mut self) {
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut self.max_vertex_attribs);
        }
    }

    /// Get a reference to the pipeline info's version.
    /// Requires that [load_version](Self::load_version) has been called in advance
    pub fn version(&self) -> &Box<&'a str> {
        self.version.as_ref().unwrap()
    }

    /// Get a reference to the pipeline info's vendor.
    /// Requires that [load_version](Self::load_vendor) has been called in advance
    pub fn vendor(&self) -> &Box<&'a str> {
        self.vendor.as_ref().unwrap()
    }

    /// Get a reference to the pipeline info's renderer.
    /// Requires that [load_version](Self::load_vendor) has been called in advance
    pub fn renderer(&self) -> &Box<&'a str> {
        self.renderer.as_ref().unwrap()
    }

    /// Get a reference to the pipeline info's max vertex attribs.
    pub fn max_vertex_attribs(&self) -> &gl::types::GLint {
        &self.max_vertex_attribs
    }
}
