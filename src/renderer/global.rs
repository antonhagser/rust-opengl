use std::sync::RwLock;

/// Structure containing data needed for global use accross rendering pipeline.
/// <br>
/// Defines structure of ex. current VertexAttribArray index
pub struct Global {
    attrib_array_index: gl::types::GLuint,
}

impl Global {
    pub fn new() -> RwLock<Global> {
        let global = Global {
            attrib_array_index: 0,
        };

        RwLock::new(global)
    }

    /// Get a reference to the global's attrib array index.
    pub fn attrib_array_index(&self) -> &gl::types::GLuint {
        &self.attrib_array_index
    }
}