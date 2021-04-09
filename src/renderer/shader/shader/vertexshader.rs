use std::ffi::CStr;

use crate::renderer::shader::kind::ShaderKind;

use super::{Shader};

pub struct VertexShader {
    id: gl::types::GLuint,
}

impl VertexShader {
    pub fn from_source(src: &CStr) -> Result<Self, String> {
        // Create shader
        let id =
            unsafe { gl::CreateShader(ShaderKind::as_opengl_enum(&ShaderKind::VertexShader)) };

        let mut vs = VertexShader { id };
        vs.compile(src)?;

        Ok(vs)
    }
}

impl Shader for VertexShader {
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn kind(&self) -> ShaderKind {
        ShaderKind::FragmentShader
    }
}