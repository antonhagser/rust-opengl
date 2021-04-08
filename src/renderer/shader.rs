use std::{ffi::{CStr, CString}, fs::File, io::Read, path::Path};

use self::kind::ShaderKind;

pub mod kind;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_file(path: &Path, kind: ShaderKind) -> Result<Shader, String> {
        let mut buf = String::new();
        File::open(&path)
            .unwrap()
            .read_to_string(&mut buf)
            .expect("Failed to read shader file");

        let code = CString::new(buf).expect("Failed to parse file");
        Shader::shader_from_source(code.as_c_str(), kind.get_gl())
    }

    fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(kind) };

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

        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}