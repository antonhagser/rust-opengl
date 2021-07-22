use std::{
    ffi::CString,
    io::Read,
};

#[derive(Clone, Copy, PartialEq)]
pub enum AssetKind {
    Shader,
    Texture,
    Video,
}

/// Asset object represents any assets within the engine such as textures and shaders
pub struct Asset {
    pub(crate) name: String,
    pub(crate) path: std::path::PathBuf,
    pub(crate) raw: Vec<u8>,
    pub(crate) kind: AssetKind,
    pub(crate) should_reload: bool,
    pub(crate) identifier: String,
    pub(crate) kind_identifier: u8,
}

impl Asset {
    pub fn new(
        name: String,
        path: std::path::PathBuf,
        kind: AssetKind,
        identifier: &str,
        kind_identifier: u8,
    ) -> Result<Self, String> {
        let mut a = Asset {
            name,
            path,
            raw: Vec::new(),
            kind,
            should_reload: false,
            identifier: identifier.to_string(),
            kind_identifier,
        };

        #[cfg(debug_assertions)]
        a.reload()?;

        Ok(a)
    }

    pub fn reload(&mut self) -> Result<(), String> {
        info!("Reloading asset data");
        let f = std::fs::File::open(self.path.clone());
        let mut f = match f {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        // Read buffer
        let mut buf = Vec::new();
        let b = f.read_to_end(&mut buf).expect("Failed reading content of file");

        // Verify buffer length
        if b == 0 {
            warn!("Buffer from reload file was empty and could therefore not be read");
        }

        // Assign buffer
        self.raw = buf;

        Ok(())
    }

    pub fn raw_to_cstr(&self) -> std::ffi::CString {
        let s = std::str::from_utf8(&self.raw).expect("Failed conversion to string");
        CString::new(s).unwrap()
    }

    /// Get a reference to the asset's kind.
    pub fn kind(&self) -> AssetKind {
        self.kind
    }

    /// Get a reference to the asset's identifier.
    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    /// Get a reference to the asset's kind identifier.
    pub fn kind_identifier(&self) -> &u8 {
        &self.kind_identifier
    }
}
