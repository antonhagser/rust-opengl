use std::ffi::CStr;

use crate::renderer::shader::kind::ShaderKind;

use super::{Shader};

pub struct FragmentShader {
    id: gl::types::GLuint,
}

impl FragmentShader {
    pub fn from_source(src: &CStr) -> Result<Self, String> {
        // Create shader
        let id =
            unsafe { gl::CreateShader(ShaderKind::as_opengl_enum(&ShaderKind::FragmentShader)) };

            let mut vs = FragmentShader { id };
            vs.compile(src)?;

        Ok(vs)
    }
}

impl Shader for FragmentShader {
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn kind(&self) -> ShaderKind {
        ShaderKind::FragmentShader
    }
}
