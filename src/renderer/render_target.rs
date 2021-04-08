use self::vertex_array::{VertexArrayObject, vertex::Vertex};

pub mod vertex_array;

// Contains data for something to render
pub struct RenderTarget<'a, VERTEX, const V: usize, const I: usize> where VERTEX: Vertex<'a> {
    vertex_array: VertexArrayObject<'a, VERTEX, V, I>,
}

impl<'a, VERTEX, const V: usize, const I: usize> RenderTarget<'a, VERTEX, V, I> where VERTEX: Vertex<'a> {
    pub fn new(vertices: [VERTEX; V], indices: [u32; I]) -> Self {
        trace!("Initializing new render target");
        let va = VertexArrayObject::new("RenderTarget", vertices, indices);

        RenderTarget {
            vertex_array: va,
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.vertex_array.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            )
        }
    }
}
