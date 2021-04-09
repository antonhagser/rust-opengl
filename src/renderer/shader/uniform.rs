use std::ffi::CStr;

pub trait Uniform {
    fn uniform1f(&self, location: &CStr, v0: f32) -> Result<(), String>;
    fn uniform2f(&self, location: &CStr, v0: f32, v1: f32) -> Result<(), String>;
    fn uniform3f(&self, location: &CStr, v0: f32, v1: f32, v2: f32) -> Result<(), String>;
    fn uniform4f(&self, location: &CStr, v0: f32, v1: f32, v2: f32, v3: f32) -> Result<(), String>;

    fn uniform1i(&self, location: &CStr, v0: i32) -> Result<(), String>;
    fn uniform2i(&self, location: &CStr, v0: i32, v1: i32) -> Result<(), String>;
    fn uniform3i(&self, location: &CStr, v0: i32, v1: i32, v2: i32) -> Result<(), String>;
    fn uniform4i(&self, location: &CStr, v0: i32, v1: i32, v2: i32, v3: i32) -> Result<(), String>;

    fn uniform1ui(&self, location: &CStr, v0: u32) -> Result<(), String>;
    fn uniform2ui(&self, location: &CStr, v0: u32, v1: u32) -> Result<(), String>;
    fn uniform3ui(&self, location: &CStr, v0: u32, v1: u32, v2: u32) -> Result<(), String>;
    fn uniform4ui(&self, location: &CStr, v0: u32, v1: u32, v2: u32, v3: u32) -> Result<(), String>;

    fn uniform1fv(&self, location: &CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String>;
    fn uniform2fv(&self, location: &CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String>;
    fn uniform3fv(&self, location: &CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String>;
    fn uniform4fv(&self, location: &CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String>;

    fn uniform1iv(&self, location: &CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String>;
    fn uniform2iv(&self, location: &CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String>;
    fn uniform3iv(&self, location: &CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String>;
    fn uniform4iv(&self, location: &CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String>;

    fn uniform1uiv(&self, location: &CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String>;
    fn uniform2uiv(&self, location: &CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String>;
    fn uniform3uiv(&self, location: &CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String>;
    fn uniform4uiv(&self, location: &CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String>;

    fn uniform_matrix2fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix3fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix4fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix2x3fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix3x2fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix2x4fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix4x2fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix3x4fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
    fn uniform_matrix4x3fv(&self, location: &CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String>;
}