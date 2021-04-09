use super::{VertexDefiner, VertexDefinerField, Vertex, VertexDefinerFieldKindConverter};

/// Defaul vertex layout
#[repr(C)]
pub struct DefaultVertex {
    pos: (f32, f32, f32),
    col: (f32, f32, f32),
}

impl DefaultVertex {
    pub fn new(pos: (f32, f32, f32), col: (f32, f32, f32)) -> Self {
        DefaultVertex { pos, col }
    }
}

impl<'a> Vertex<'a> for DefaultVertex {
    fn get_definition() -> VertexDefiner<'a> {
        let fields = vec![
            VertexDefinerField::new("Position", 3, std::mem::size_of::<f32>() * 3, f32::get_vertex_field_kind()),
            VertexDefinerField::new("Color", 3, std::mem::size_of::<f32>() * 3, f32::get_vertex_field_kind()),
        ];

        VertexDefiner::new(fields)
    }
}