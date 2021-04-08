use crate::renderer::buffer::Buffer;

pub struct IndexBuffer<const I: usize> {
    id: gl::types::GLuint,
    indices: [u32; I],
}

impl<const I: usize> IndexBuffer<I> {
    pub fn new(indices: [u32; I]) -> Self {
        let mut idb = IndexBuffer { id: 0, indices };

        trace!("Initializing vertexbuffer");
        unsafe {
            gl::GenBuffers(1, &mut idb.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, idb.id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (idb.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                idb.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        idb
    }

    /// Get a reference to the index buffer's id.
    pub fn id(&self) -> &gl::types::GLuint {
        &self.id
    }

    /// Get a reference to the index buffer's indices.
    pub fn indices(&self) -> &[u32; I] {
        &self.indices
    }
}

impl<const I: usize> Buffer for IndexBuffer<I>
{
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl<const I: usize> Drop for IndexBuffer<I> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
