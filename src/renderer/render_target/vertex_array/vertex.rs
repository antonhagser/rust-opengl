pub use default::DefaultVertex;

pub mod default;

/// Struct which owns vertices, and operate on them.
pub struct VerticesArray<T, const V: usize> {
    array: [T; V],
}

impl<T, const V: usize> VerticesArray<T, V> {
    pub fn new(array: [T; V]) -> Self {
        VerticesArray { array }
    }

    /// Get size of vertices array
    fn get_inner_size(&self) -> usize {
        std::mem::size_of_val(&self.array)
    }

    /// Get pointer to inner array
    fn as_ptr(&self) -> *const T {
        self.array.as_ptr()
    }
}

/// Defines a Vertex field for a generic vertex
pub struct VertexDefinerField<'a> {
    name: &'a str,
    kind: VertexDefinerFieldKind,

    /// The count of values inside field inside Vertex
    count: usize,

    /// The byte size of one field
    size: usize,
}

impl<'a> VertexDefinerField<'a> {
    pub fn new(name: &'a str, count: usize, size: usize, kind: VertexDefinerFieldKind) -> Self {
        VertexDefinerField {
            name,
            kind,
            count,
            size,
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
    pub fn kind(&self) -> &VertexDefinerFieldKind {
        &self.kind
    }
}

pub enum VertexDefinerFieldKind {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
}

impl VertexDefinerFieldKind {
    pub fn get_opengl_enum(&self) -> gl::types::GLenum {
        match self {
            VertexDefinerFieldKind::Byte => gl::BYTE,
            VertexDefinerFieldKind::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexDefinerFieldKind::Short => gl::SHORT,
            VertexDefinerFieldKind::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexDefinerFieldKind::Int => gl::INT,
            VertexDefinerFieldKind::UnsignedInt => gl::UNSIGNED_INT,
            VertexDefinerFieldKind::Float => gl::FLOAT,
        }
    }
}

// Trait for auto conversion
trait VertexDefinerFieldKindConverter {
    fn get_vertex_field_kind() -> VertexDefinerFieldKind;
}

impl VertexDefinerFieldKindConverter for f32 {
    fn get_vertex_field_kind() -> VertexDefinerFieldKind {
        VertexDefinerFieldKind::Float
    }
}

impl VertexDefinerFieldKindConverter for i32 {
    fn get_vertex_field_kind() -> VertexDefinerFieldKind {
        VertexDefinerFieldKind::Int
    }
}

impl VertexDefinerFieldKindConverter for u32 {
    fn get_vertex_field_kind() -> VertexDefinerFieldKind {
        VertexDefinerFieldKind::UnsignedInt
    }
}

/// Defines a vertex and explains it's inner data.
/// Used for vertex attrib pointers.
pub struct VertexDefiner<'a> {
    fields: Vec<VertexDefinerField<'a>>,
}

impl<'a> VertexDefiner<'a> {
    pub fn new(fields: Vec<VertexDefinerField<'a>>) -> Self {
        VertexDefiner { fields }
    }

    pub fn full_size(&self) -> usize {
        let mut size = 0;
        for f in self.fields.iter() {
            size += f.size();
        }

        return size;
    }

    /// Get a reference to the vertex definer's fields as an iterator.
    pub fn fields(&self) -> std::slice::Iter<'_, VertexDefinerField<'_>> {
        self.fields.iter()
    }
}

// Todo: Refactor with associated types defaults once available
pub trait Vertex<'a> {
    /// Get the inner defintion of the vertex data structure. <br>
    /// **Fields shall have the same name as the corresponding shader variable**
    fn get_definition() -> VertexDefiner<'a>;
}
