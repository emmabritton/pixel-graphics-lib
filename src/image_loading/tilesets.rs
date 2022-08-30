use crate::image::Image;
use crate::image_loading::ImageWrapperError::*;
use crate::image_loading::{convert_image, ImageWrapperError};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

/// BasicTileset can be used for simple tile sets/atlas
///
/// The format must be:
/// {
///     name: string, optional
///     image_file: string, path
///     tile_sizes:
///         width: integer, px
///         height: integer, px
///     tiles: [
///         id: string
///         x: integer, coord
///         y: integer, coord
///     ]
/// }
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct BasicTileset {
    name: Option<String>,
    image_file: String,
    tile_sizes: TilesetSize,
    tiles: Vec<TilesetTile>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
struct TilesetSize {
    width: usize,
    height: usize,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
struct TilesetTile {
    id: String,
    x: usize,
    y: usize,
}

impl BasicTileset {
    /// See [BasicTileset] for format
    ///
    /// Returns tile ids mapped to Images
    pub fn load_from_file(path: &str) -> Result<HashMap<String, Image>, ImageWrapperError> {
        let mut map = HashMap::new();

        let json = fs::read_to_string(path).map_err(TilesetFileError)?;
        let tileset: BasicTileset =
            serde_json::from_str(&json).map_err(|err| TilesetFormatError(err.to_string()))?;
        let tileset_image = image::open(&tileset.image_file).map_err(ImageFileError)?;

        for tile in tileset.tiles {
            let image = convert_image(tileset_image.crop_imm(
                (tile.x * tileset.tile_sizes.width) as u32,
                (tile.y * tileset.tile_sizes.height) as u32,
                tileset.tile_sizes.width as u32,
                tileset.tile_sizes.height as u32,
            ))?;
            map.insert(tile.id, image);
        }

        Ok(map)
    }
}
