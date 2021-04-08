
pub enum ShaderKind {
    VertexShader,
    FragmentShader,
}

impl ShaderKind {
    pub fn get_gl(self) -> gl::types::GLenum {
        match self {
            ShaderKind::VertexShader => gl::VERTEX_SHADER,
            ShaderKind::FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}