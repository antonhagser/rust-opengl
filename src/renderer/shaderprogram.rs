use std::{ffi::CString, path::Path};

use super::shader::{kind::ShaderKind, Shader};

#[allow(dead_code)]
pub struct ShaderProgram<const T: usize> {
    id: gl::types::GLuint,
    shaders: [Shader; T],
}

impl<const T: usize> ShaderProgram<T> {
    pub fn load_shaders_from_file(
        vert: &Path,
        frag: &Path,
    ) -> Result<ShaderProgram<2_usize>, String> {
        let vertex = Shader::from_file(&vert, ShaderKind::VertexShader)?;
        let fragment = Shader::from_file(&frag, ShaderKind::FragmentShader)?;

        Ok(ShaderProgram::from_shaders([vertex, fragment])?)
    }

    pub fn from_shaders(shaders: [Shader; T]) -> Result<ShaderProgram<T>, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders.iter() {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders.iter() {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(ShaderProgram {
            id: program_id,
            shaders,
        })
    }

    /// Get a reference to the shader program's id.
    pub fn id(&self) -> &gl::types::GLuint {
        &self.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl<const T: usize> Drop for ShaderProgram<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
