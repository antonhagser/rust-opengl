use self::{indexbuffer::IndexBuffer, vertexbuffer::VertexBuffer};
use crate::renderer::buffer::Buffer;

mod vertex;

pub use vertex::Vertex;
pub use vertex::VertexDefiner;
pub use vertex::VertexField;
pub use vertex::VertexFieldKind;
pub use vertex::DefaultVertex;

pub mod indexbuffer;
pub mod vertexbuffer;

/// VertexArrayObject stores data about vertices and indices, it owns the vertex array
/// and therefore also owns the vertex buffer and index buffer
pub struct VertexArrayObject<'a, VERTEX, const V: usize, const I: usize>
where
    VERTEX: Vertex<'a>,
{
    identifier: &'a str,
    vertex_array: gl::types::GLuint,
    vertex_buffer: VertexBuffer<'a, VERTEX, V>,
    index_buffer: IndexBuffer<I>,
}

impl<'a, VERTEX, const V: usize, const I: usize> VertexArrayObject<'a, VERTEX, V, I>
where
    VERTEX: Vertex<'a>,
{
    pub fn new(identifier: &'a str, vertices: [VERTEX; V], indices: [u32; I]) -> Self {
        trace!("Creating vertex array with identifier {}", identifier);
        // Generate and assign vertex array
        let mut vertex_array = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
        }

        // Define vertex array object
        let vao = VertexArrayObject {
            identifier,
            vertex_array,
            vertex_buffer: VertexBuffer::new(vertices),
            index_buffer: IndexBuffer::new(indices),
        };

        // Rebind buffers
        vao.unbind_all();
        vao.bind_all();

        vao
    }

    /// Bind vertex array
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
        }
    }

    // Unbind vertex array
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Binds all buffers with vertex array. Used for resetting all bindings
    pub fn bind_all(&self) {
        self.bind();
        self.vertex_buffer.bind();
        self.index_buffer.bind();
    }

    /// Unbinds all buffers with vertex array. Used for resetting all bindings
    pub fn unbind_all(&self) {
        self.unbind();
        self.vertex_buffer.unbind();
        self.index_buffer.unbind();
    }

    /// Get a reference to the vertex array object's identifier.
    pub fn identifier(&self) -> &&'a str {
        &self.identifier
    }

    /// Get a reference to the vertex array object's vertex buffer.
    pub fn vertex_buffer(&self) -> &VertexBuffer<'a, VERTEX, V> {
        &self.vertex_buffer
    }

    /// Get a reference to the vertex array object's index buffer.
    pub fn index_buffer(&self) -> &IndexBuffer<I> {
        &self.index_buffer
    }
}
