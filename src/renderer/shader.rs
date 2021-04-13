use std::{collections::HashMap, ffi::CString, sync::{Arc, RwLock}};

use crate::assets::Asset;

use super::render_target::vertex_array::{Vertex, VertexDefiner};

pub use kind::ShaderKind;
pub use shader::{FragmentShader, Shader, VertexShader};
pub use uniform::Uniform;

mod kind;
mod shader;
mod uniform;

pub struct ShaderProgram<'a> {
    shaders: HashMap<ShaderKind, Box<dyn Shader>>,
    // shaders: Vec<Box<dyn Shader>>,
    locations: HashMap<&'a str, gl::types::GLuint>,
    id: gl::types::GLuint,
    definer: Option<VertexDefiner<'a>>,
}

impl<'a> ShaderProgram<'a> {
    pub fn new<T>(shaders: Vec<Box<dyn Shader>>) -> Result<Self, String>
    where
        T: Vertex<'a>,
    {
        let mut hash = HashMap::new();
        for s in shaders {
            hash.insert(s.kind(), s);
        }

        let mut sp = ShaderProgram {
            shaders: hash,
            locations: HashMap::new(),
            id: 0,
            definer: None,
        };

        // Create shader program
        trace!("Creating shader program");
        unsafe {
            sp.id = gl::CreateProgram();
        }

        let def = T::get_definition();
        sp.definer = Some(def);
        sp.internal_new()?;

        Ok(sp)
    }

    fn internal_new(&mut self) -> Result<(), String> {
        // Attach the shader objects to the program
        for shader in self.shaders.iter() {
            unsafe {
                gl::AttachShader(self.id(), shader.1.id());
            }
        }

        for (i, field) in self.definer.as_ref().unwrap().fields().enumerate() {
            let raw = std::ffi::CString::new(field.name().to_string())
                .expect("Failed at conveting shader to CString");
            unsafe {
                gl::BindAttribLocation(self.id, i as gl::types::GLuint, raw.as_c_str().as_ptr());
            }
        }

        // Link shaders to shaderprogram
        self.link()?;

        // Detach shaders to allow OpenGL to delete shaders
        for shader in self.shaders.iter() {
            unsafe {
                gl::DetachShader(self.id(), shader.1.id());
            }
        }

        Ok(())
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
            gl::GetProgramiv(self.id(), gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // Get info about non successfull error
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(self.id(), gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetProgramInfoLog(
                    self.id(),
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
        unsafe { gl::UseProgram(self.id()) }
    }

    // Unbind program
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn get_uniform_location(&mut self, location: &'a str) -> Result<gl::types::GLuint, String> {
        let loc = self.locations.get(location);
        match loc {
            Some(l) => Ok(*l),
            None => {
                let location = CString::new(location);
                let location = match location {
                    Ok(c) => c,
                    Err(e) => return Err(format!("Failed to parse location {}", e)),
                };
                let loc = unsafe { gl::GetUniformLocation(self.id(), location.as_ptr()) };

                Ok(loc as gl::types::GLuint)
            }
        }
    }

    /// Get a reference to the shader program's id.
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// Reload shaders
    pub fn reload(&mut self, asset: Arc<RwLock<Asset>>) {
        trace!("Triggered internal reload of shader-program");
        for s in self.shaders.iter_mut() {
            let asset = asset.as_ref().read().expect("Tried to get asset reader");
            let kind = ShaderKind::from_u8(*asset.kind_identifier());
            if *s.0 == kind {
                let raw = asset.raw_to_cstr();
                s.1.recompile(raw.as_c_str());
            }
        }

        self.internal_new().expect("Failed relinking shader program after hotreload");
    }
}

impl<'a> Uniform<'a> for ShaderProgram<'a> {
    fn uniform1f(&mut self, location: &'a str, v0: f32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1f(loc as gl::types::GLint, v0);
        }

        Ok(())
    }

    fn uniform2f(&mut self, location: &'a str, v0: f32, v1: f32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2f(loc as gl::types::GLint, v0, v1);
        }

        Ok(())
    }

    fn uniform3f(&mut self, location: &'a str, v0: f32, v1: f32, v2: f32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3f(loc as gl::types::GLint, v0, v1, v2);
        }

        Ok(())
    }

    fn uniform4f(
        &mut self,
        location: &'a str,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4f(loc as gl::types::GLint, v0, v1, v2, v3);
        }

        Ok(())
    }

    fn uniform1i(&mut self, location: &'a str, v0: i32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1i(loc as gl::types::GLint, v0);
        }

        Ok(())
    }

    fn uniform2i(&mut self, location: &'a str, v0: i32, v1: i32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2i(loc as gl::types::GLint, v0, v1);
        }

        Ok(())
    }

    fn uniform3i(&mut self, location: &'a str, v0: i32, v1: i32, v2: i32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3i(loc as gl::types::GLint, v0, v1, v2);
        }

        Ok(())
    }

    fn uniform4i(
        &mut self,
        location: &'a str,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4i(loc as gl::types::GLint, v0, v1, v2, v3);
        }

        Ok(())
    }

    fn uniform1ui(&mut self, location: &'a str, v0: u32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1ui(loc as gl::types::GLint, v0);
        }

        Ok(())
    }

    fn uniform2ui(&mut self, location: &'a str, v0: u32, v1: u32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2ui(loc as gl::types::GLint, v0, v1);
        }

        Ok(())
    }

    fn uniform3ui(&mut self, location: &'a str, v0: u32, v1: u32, v2: u32) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3ui(loc as gl::types::GLint, v0, v1, v2);
        }

        Ok(())
    }

    fn uniform4ui(
        &mut self,
        location: &'a str,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4ui(loc as gl::types::GLint, v0, v1, v2, v3);
        }

        Ok(())
    }

    fn uniform1fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [f32; 1],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1fv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform2fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [f32; 2],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2fv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform3fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [f32; 3],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3fv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform4fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [f32; 4],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4fv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform1iv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [i32; 1],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1iv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform2iv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [i32; 2],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2iv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform3iv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [i32; 3],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3iv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform4iv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [i32; 4],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4iv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform1uiv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [u32; 1],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform1uiv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform2uiv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [u32; 2],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform2uiv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform3uiv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [u32; 3],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform3uiv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform4uiv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        value: [u32; 4],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::Uniform4uiv(loc as gl::types::GLint, count, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix2fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 4],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix2fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix3fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 9],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix3fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix4fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 16],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix4fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix2x3fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 6],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix2x3fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix3x2fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 6],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix3x2fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix2x4fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 8],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix2x4fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix4x2fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 8],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix4x2fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix3x4fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 12],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix3x4fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }

    fn uniform_matrix4x3fv(
        &mut self,
        location: &'a str,
        count: gl::types::GLsizei,
        transpose: gl::types::GLboolean,
        value: [f32; 12],
    ) -> Result<(), String> {
        let loc = self.get_uniform_location(location)?;
        unsafe {
            gl::UniformMatrix4x3fv(loc as gl::types::GLint, count, transpose, value.as_ptr());
        }

        Ok(())
    }
}

impl<'a> Drop for ShaderProgram<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id());
            for shader in self.shaders.iter() {
                drop(shader);
            }
        }
    }
}
