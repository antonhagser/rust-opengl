pub enum ShaderKind {
    VertexShader,
    TessControlShader,
    TessEvaluationShader,
    GeometryShader,
    FragmentShader,
    ComputeShader,
}

impl ShaderKind {
    pub fn as_opengl_enum(&self) -> gl::types::GLenum {
        match &self {
            ShaderKind::VertexShader => gl::VERTEX_SHADER,
            ShaderKind::TessControlShader => gl::TESS_CONTROL_SHADER,
            ShaderKind::TessEvaluationShader => gl::TESS_EVALUATION_SHADER,
            ShaderKind::GeometryShader => gl::GEOMETRY_SHADER,
            ShaderKind::FragmentShader => gl::FRAGMENT_SHADER,
            ShaderKind::ComputeShader => gl::COMPUTE_SHADER
        }
    }
}