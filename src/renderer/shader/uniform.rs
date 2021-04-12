#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait Uniform<'a> {
    fn uniform1f(&mut self, location: &'a str, v0: f32) -> Result<(), String>;
    fn uniform2f(&mut self, location: &'a str, v0: f32, v1: f32) -> Result<(), String>;
    fn uniform3f(&mut self, location: &'a str, v0: f32, v1: f32, v2: f32) -> Result<(), String>;
    fn uniform4f(&mut self, location: &'a str, v0: f32, v1: f32, v2: f32, v3: f32) -> Result<(), String>;

    fn uniform1i(&mut self, location: &'a str, v0: i32) -> Result<(), String>;
    fn uniform2i(&mut self, location: &'a str, v0: i32, v1: i32) -> Result<(), String>;
    fn uniform3i(&mut self, location: &'a str, v0: i32, v1: i32, v2: i32) -> Result<(), String>;
    fn uniform4i(&mut self, location: &'a str, v0: i32, v1: i32, v2: i32, v3: i32) -> Result<(), String>;

    fn uniform1ui(&mut self, location: &'a str, v0: u32) -> Result<(), String>;
    fn uniform2ui(&mut self, location: &'a str, v0: u32, v1: u32) -> Result<(), String>;
    fn uniform3ui(&mut self, location: &'a str, v0: u32, v1: u32, v2: u32) -> Result<(), String>;
    fn uniform4ui(&mut self, location: &'a str, v0: u32, v1: u32, v2: u32, v3: u32) -> Result<(), String>;

    fn uniform1fv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [f32; 1]) -> Result<(), String>;
    fn uniform2fv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [f32; 2]) -> Result<(), String>;
    fn uniform3fv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [f32; 3]) -> Result<(), String>;
    fn uniform4fv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [f32; 4]) -> Result<(), String>;

    fn uniform1iv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [i32; 1]) -> Result<(), String>;
    fn uniform2iv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [i32; 2]) -> Result<(), String>;
    fn uniform3iv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [i32; 3]) -> Result<(), String>;
    fn uniform4iv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [i32; 4]) -> Result<(), String>;

    fn uniform1uiv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [u32; 1]) -> Result<(), String>;
    fn uniform2uiv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [u32; 2]) -> Result<(), String>;
    fn uniform3uiv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [u32; 3]) -> Result<(), String>;
    fn uniform4uiv(&mut self, location: &'a str, count: gl::types::GLsizei, value: [u32; 4]) -> Result<(), String>;

    fn uniform_matrix2fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 4]) -> Result<(), String>;
    fn uniform_matrix3fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 9]) -> Result<(), String>;
    fn uniform_matrix4fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 16]) -> Result<(), String>;
    fn uniform_matrix2x3fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 6]) -> Result<(), String>;
    fn uniform_matrix3x2fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 6]) -> Result<(), String>;
    fn uniform_matrix2x4fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 8]) -> Result<(), String>;
    fn uniform_matrix4x2fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 8]) -> Result<(), String>;
    fn uniform_matrix3x4fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 12]) -> Result<(), String>;
    fn uniform_matrix4x3fv(&mut self, location: &'a str, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: [f32; 12]) -> Result<(), String>;
}
