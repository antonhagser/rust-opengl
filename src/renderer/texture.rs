use self::image::{Format, Image};

pub mod image;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub data: Option<Vec<u8>>,

    id: gl::types::GLuint,
}

impl Texture {
    // creates a new texture with the given width and height
    pub fn new(width: u32, height: u32, format: Format) -> Texture {
        Texture {
            width,
            height,
            format,
            data: None,

            id: 0,
        }
    }

    // assigns data to the texture
    pub fn with_data(mut self, data: Vec<u8>) -> Texture {
        self.data = Some(data);
        self
    }

    // initializes a new texture from an image
    pub fn from_image(image: Image) -> Texture {
        Texture {
            width: image.width,
            height: image.height,
            format: image.format,
            data: image.data,

            id: 0,
        }
    }

    // builds the texture
    pub fn build(mut self) -> Texture {
        // set opengl texture parameters
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::MIRRORED_REPEAT as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::MIRRORED_REPEAT as gl::types::GLint,
            );
        }

        // assign texture scaling filters
        // configure mipmap
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as gl::types::GLint,
            );
        }

        // generate and bind texture
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }

        // set correct internal format for opengl
        let format = match self.format {
            Format::RGB8 => gl::RGB,
            Format::RGBA8 => gl::RGBA,
            _ => unreachable!(),
        };

        // upload texture data and generate mipmap
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as gl::types::GLint,
                self.width as gl::types::GLint,
                self.height as gl::types::GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                self.data.as_ref().unwrap().as_ptr() as *const gl::types::GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        self
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
