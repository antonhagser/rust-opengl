use std::ffi::{CStr, CString};

use super::kind::ShaderKind;

mod vertexshader;
mod fragmentshader;

pub use vertexshader::VertexShader;
pub use fragmentshader::FragmentShader;

pub trait Shader {
    fn id(&self) -> gl::types::GLuint;
    fn kind(&self) -> ShaderKind;

    /// Compiles the program from source
    fn compile(&mut self, source: &CStr) -> Result<(), String> {
        let id = self.id();

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            // Get length of error
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

            // Fetch error
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(())
    }
}