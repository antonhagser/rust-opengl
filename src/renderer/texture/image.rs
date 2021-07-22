use std::path::PathBuf;

use image::io::Reader as ImageReader;

#[derive(Debug)]
pub enum Format {
    Unknown = 0,
    RGB8 = 1,
    RGBA8 = 2,
}

#[derive(Debug)]
pub struct Image {
    pub(crate) format: Format,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Option<Vec<u8>>,
}

impl Image {
    pub fn read_image(path: PathBuf) -> Result<Image, Box<dyn std::error::Error>> {
        let mut image = Image {
            format: Format::Unknown,
            width: 0,
            height: 0,
            data: None,
        };

        let img = ImageReader::open(path)?.decode()?;
        match img {
            image::DynamicImage::ImageRgb8(img) => {
                (image.width, image.height) = img.dimensions();
                image.format = Format::RGB8;
                image.data = Some(img.to_vec());
            }
            image::DynamicImage::ImageRgba8(img) => {
                (image.width, image.height) = img.dimensions();
                image.format = Format::RGBA8;
                image.data = Some(img.to_vec());
            }
            _ => {
                let img = img.to_rgba8();
                (image.width, image.height) = img.dimensions();
                image.format = Format::RGBA8;
                image.data = Some(img.to_vec());
            }
        };

        Ok(image)
    }
}
