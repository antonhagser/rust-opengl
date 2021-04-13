#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => ShaderKind::VertexShader,
            1 => ShaderKind::TessControlShader,
            2 => ShaderKind::TessEvaluationShader,
            3 => ShaderKind::GeometryShader,
            4 => ShaderKind::FragmentShader,
            5 => ShaderKind::ComputeShader,
            _ => ShaderKind::ComputeShader
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ShaderKind::VertexShader => 0,
            ShaderKind::TessControlShader => 1,
            ShaderKind::TessEvaluationShader => 2,
            ShaderKind::GeometryShader => 3,
            ShaderKind::FragmentShader => 4,
            ShaderKind::ComputeShader => 5,
        }
    }
}