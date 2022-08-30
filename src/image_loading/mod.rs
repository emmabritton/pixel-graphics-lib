pub mod tilesets;

use crate::color::Color;
use crate::image::Image;
use crate::image_loading::ImageWrapperError::GraphicsLibError;
use crate::GraphicsError;
use image::DynamicImage;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageWrapperError {
    #[error("Creating image")]
    GraphicsLibError(#[from] GraphicsError),
    #[error("Creating tile {0}")]
    TileError(String, #[source] GraphicsError),
    #[error("Tileset file error")]
    TilesetFileError(#[from] std::io::Error),
    #[error("Tileset format error: {0}")]
    TilesetFormatError(String),
    #[error("Tileset image reading error")]
    ImageFileError(#[from] image::ImageError),
}

/// Load a file
pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Image, ImageWrapperError> {
    convert_image(image::open(path)?)
}

//Convert a `DynamicImage` from the `image` crate into an `Image`
pub fn convert_image(image: DynamicImage) -> Result<Image, ImageWrapperError> {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let pixels = image
        .into_rgba8()
        .chunks_exact(4)
        .map(|px| Color::rgba(px[0], px[1], px[2], px[3]))
        .collect();

    Image::new(pixels, width, height).map_err(GraphicsLibError)
}
