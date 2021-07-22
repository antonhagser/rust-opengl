use crate::renderer::buffer::Buffer;

use super::vertex::Vertex;

pub struct VertexBuffer<'a, VERTEX, const V: usize>
where
    VERTEX: Vertex<'a>,
{
    id: gl::types::GLuint,
    _identifier: &'a u8,
    vertices: [VERTEX; V],
}

impl<'a, VERTEX, const V: usize> VertexBuffer<'a, VERTEX, V>
where
    VERTEX: Vertex<'a>,
{
    pub fn new(vertices: [VERTEX; V]) -> Self {
        let mut vb = VertexBuffer {
            id: 0,
            _identifier: &0,
            vertices,
        };

        // Generate the buffers
        trace!("Initializing vertexbuffer");
        unsafe {
            gl::GenBuffers(1, &mut vb.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb.id);

            // Calculate buffer size and assign data
            let size = vb.calculate_vertices_size();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as gl::types::GLsizeiptr,
                vb.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }

        // Assign buffer vertex attributes
        vb.assign_vertex_attrib_pointer();

        vb
    }

    /// Calculates the inner byte size of all vertices
    fn calculate_vertices_size(&self) -> usize {
        VERTEX::get_definition()
            .size()
            * self.vertices().len()
    }

    /// Assigns vertex attrib pointers and enables index
    pub fn assign_vertex_attrib_pointer(&self) {
        trace!("Assigning vertex attribute pointers");
        let mut index = 0;
        let mut stride = 0;

        // Fetch definition of vertex
        let definer = VERTEX::get_definition();

        // Todo: Support vertex attrib pointer with size larger than 4
        for field in definer.fields() {
            unsafe {
                gl::VertexAttribPointer(
                    index,
                    field.count() as gl::types::GLint,
                    field.kind().get_opengl_enum(),
                    gl::FALSE,
                    definer.size() as gl::types::GLint,
                    stride as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(index);
            }

            stride += field.size();
            index += 1;
        }
    }

    /// Get a reference to the index buffer's id.
    pub fn id(&self) -> &gl::types::GLuint {
        &self.id
    }

    /// Get a reference to the vertex buffer's vertices.
    pub fn vertices(&self) -> &[VERTEX; V] {
        &self.vertices
    }
}

impl<'a, VERTEX, const V: usize> Buffer for VertexBuffer<'a, VERTEX, V>
where
    VERTEX: Vertex<'a>,
{
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl<'a, VERTEX, const V: usize> Drop for VertexBuffer<'a, VERTEX, V>
where
    VERTEX: Vertex<'a>,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
