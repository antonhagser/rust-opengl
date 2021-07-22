use super::{VertexDefiner, VertexField, Vertex};

/// Defaul vertex layout
#[derive(Debug)]
pub struct DefaultVertex {
    pos: (f32, f32, f32),
    col: (f32, f32, f32),
    tex: (f32, f32)
}

impl DefaultVertex {
    pub fn new(pos: (f32, f32, f32), col: (f32, f32, f32), tex: (f32, f32)) -> Self {
        DefaultVertex { pos, col, tex }
    }
}

impl<'a> Vertex<'a> for DefaultVertex {
    fn get_definition() -> VertexDefiner<'a> {
        let fields = vec![
            VertexField::new::<f32>("Position", 3),
            VertexField::new::<f32>("Color", 3),
            VertexField::new::<f32>("TexCoord", 2),
        ];

        VertexDefiner::new(fields)
    }
}