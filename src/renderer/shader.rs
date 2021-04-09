use std::ffi::CString;

use self::{shader::Shader, uniform::Uniform};

use super::render_target::vertex_array::Vertex;

pub mod kind;
pub mod shader;

mod uniform;

pub struct ShaderProgram {
    shaders: Vec<Box<dyn Shader>>,
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn new<'a, T>(shaders: Vec<Box<dyn Shader>>) -> Result<Self, String>
    where
        T: Vertex<'a>,
    {
        let mut sp = ShaderProgram { shaders, id: 0 };

        // Create shader program
        trace!("Creating shader program");
        unsafe {
            sp.id = gl::CreateProgram();
        }

        // Attach the shader objects to the program
        for shader in sp.shaders.iter() {
            unsafe {
                gl::AttachShader(*sp.id(), shader.id());
            }
        }

        for (i, field) in T::get_definition().fields().enumerate() {
            let raw = std::ffi::CString::new(field.name().to_string())
                .expect("Failed at conveting shader to CString");
            unsafe {
                gl::BindAttribLocation(sp.id, i as gl::types::GLuint, raw.as_c_str().as_ptr());
            }
        }

        // Link shaders to shaderprogram
        sp.link()?;

        // Detach shaders to allow OpenGL to delete shaders
        for shader in sp.shaders.iter() {
            unsafe {
                gl::DetachShader(*sp.id(), shader.id());
            }
        }

        Ok(sp)
    }

    /// Link shader program
    pub fn link(&self) -> Result<(), String> {
        // Link program
        unsafe {
            gl::LinkProgram(self.id);
        }

        // Get linking status
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(*self.id(), gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // Get info about non successfull error
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(*self.id(), gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetProgramInfoLog(
                    *self.id(),
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(())
    }

    /// Bind program
    pub fn bind(&self) {
        unsafe { gl::UseProgram(*self.id()) }
    }

    // Unbind program
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Get a reference to the shader program's id.
    pub fn id(&self) -> &gl::types::GLuint {
        &self.id
    }
}

impl Uniform for ShaderProgram {
    fn uniform1f(&self, location: &std::ffi::CStr, v0: f32) -> Result<(), String> {
        todo!()
    }

    fn uniform2f(&self, location: &std::ffi::CStr, v0: f32, v1: f32) -> Result<(), String> {
        todo!()
    }

    fn uniform3f(&self, location: &std::ffi::CStr, v0: f32, v1: f32, v2: f32) -> Result<(), String> {
        todo!()
    }

    fn uniform4f(&self, location: &std::ffi::CStr, v0: f32, v1: f32, v2: f32, v3: f32) -> Result<(), String> {
        todo!()
    }

    fn uniform1i(&self, location: &std::ffi::CStr, v0: i32) -> Result<(), String> {
        todo!()
    }

    fn uniform2i(&self, location: &std::ffi::CStr, v0: i32, v1: i32) -> Result<(), String> {
        todo!()
    }

    fn uniform3i(&self, location: &std::ffi::CStr, v0: i32, v1: i32, v2: i32) -> Result<(), String> {
        todo!()
    }

    fn uniform4i(&self, location: &std::ffi::CStr, v0: i32, v1: i32, v2: i32, v3: i32) -> Result<(), String> {
        todo!()
    }

    fn uniform1ui(&self, location: &std::ffi::CStr, v0: u32) -> Result<(), String> {
        todo!()
    }

    fn uniform2ui(&self, location: &std::ffi::CStr, v0: u32, v1: u32) -> Result<(), String> {
        todo!()
    }

    fn uniform3ui(&self, location: &std::ffi::CStr, v0: u32, v1: u32, v2: u32) -> Result<(), String> {
        todo!()
    }

    fn uniform4ui(&self, location: &std::ffi::CStr, v0: u32, v1: u32, v2: u32, v3: u32) -> Result<(), String> {
        todo!()
    }

    fn uniform1fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform2fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform3fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform4fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform1iv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String> {
        todo!()
    }

    fn uniform2iv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String> {
        todo!()
    }

    fn uniform3iv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String> {
        todo!()
    }

    fn uniform4iv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const i32) -> Result<(), String> {
        todo!()
    }

    fn uniform1uiv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String> {
        todo!()
    }

    fn uniform2uiv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String> {
        todo!()
    }

    fn uniform3uiv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String> {
        todo!()
    }

    fn uniform4uiv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, value: *const u32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix2fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix3fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix4fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix2x3fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix3x2fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix2x4fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix4x2fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix3x4fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }

    fn uniform_matrix4x3fv(&self, location: &std::ffi::CStr, count: gl::types::GLsizei, transpose: gl::types::GLboolean, value: *const f32) -> Result<(), String> {
        todo!()
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(*self.id());
            for shader in self.shaders.iter() {
                drop(shader);
            }
        }
    }
}
