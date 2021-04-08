use super::{VertexDefiner, VertexDefinerField, Vertex, VertexDefinerFieldKindConverter};

/// Defaul vertex layout
pub struct DefaultVertex {
    pos: (f32, f32, f32),
}

impl DefaultVertex {
    pub fn new(pos: (f32, f32, f32)) -> Self {
        DefaultVertex { pos }
    }
}

impl<'a> Vertex<'a> for DefaultVertex {
    fn get_definition(&self) -> VertexDefiner<'a> {
        let fields = vec![
            VertexDefinerField::new("pos", 3, &self.pos, f32::get_vertex_field_kind()),
        ];

        VertexDefiner::new(fields)
    }
}