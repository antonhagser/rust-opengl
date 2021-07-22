pub use default::DefaultVertex;

pub mod default;

/// Defines a Vertex field for a generic vertex
pub struct VertexField<'a> {
    name: &'a str,
    kind: VertexFieldKind,

    /// The count of values inside field inside Vertex
    count: usize,

    /// The byte size of one field
    size: usize,
}

impl<'a> VertexField<'a> {
    pub fn new<T>(name: &'a str, count: usize) -> Self where T: VertexFieldKindConverter {
        VertexField {
            name,
            kind: T::get_vertex_field_kind(),
            count,
            size: std::mem::size_of::<T>() * count,
        }
    }

    /// Get the vertex definer field's size.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the vertex definer field's count.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get a reference to the vertex definer field's name.
    pub fn name(&self) -> &&'a str {
        &self.name
    }

    /// Get a reference to the vertex definer field's kind.
    pub fn kind(&self) -> &VertexFieldKind {
        &self.kind
    }
}

pub enum VertexFieldKind {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
}

impl VertexFieldKind {
    pub fn get_opengl_enum(&self) -> gl::types::GLenum {
        match self {
            VertexFieldKind::Byte => gl::BYTE,
            VertexFieldKind::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexFieldKind::Short => gl::SHORT,
            VertexFieldKind::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexFieldKind::Int => gl::INT,
            VertexFieldKind::UnsignedInt => gl::UNSIGNED_INT,
            VertexFieldKind::Float => gl::FLOAT,
        }
    }
}

// Trait for auto conversion
pub trait VertexFieldKindConverter {
    fn get_vertex_field_kind() -> VertexFieldKind;
}

impl VertexFieldKindConverter for f32 {
    fn get_vertex_field_kind() -> VertexFieldKind {
        VertexFieldKind::Float
    }
}

impl VertexFieldKindConverter for i32 {
    fn get_vertex_field_kind() -> VertexFieldKind {
        VertexFieldKind::Int
    }
}

impl VertexFieldKindConverter for u32 {
    fn get_vertex_field_kind() -> VertexFieldKind {
        VertexFieldKind::UnsignedInt
    }
}

/// Defines a vertex and explains it's inner data.
/// Used for vertex attrib pointers.
pub struct VertexDefiner<'a> {
    fields: Vec<VertexField<'a>>,
}

impl<'a> VertexDefiner<'a> {
    pub fn new(fields: Vec<VertexField<'a>>) -> Self {
        VertexDefiner { fields }
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        for f in self.fields.iter() {
            size += f.size();
        }

        return size;
    }

    /// Get a reference to the vertex definer's fields as an iterator.
    pub fn fields(&self) -> std::slice::Iter<'_, VertexField<'_>> {
        self.fields.iter()
    }
}

// Todo: Refactor with associated types defaults once available
pub trait Vertex<'a> {
    /// Get the inner defintion of the vertex data structure. <br>
    /// **Fields shall have the same name as the corresponding shader variable**
    fn get_definition() -> VertexDefiner<'a>;
}
