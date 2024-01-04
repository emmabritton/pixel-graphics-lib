use crate::ui::prelude::*;
use buffer_graphics_lib::prelude::*;
use std::fmt::Debug;

#[cfg(feature = "directories")]
pub mod load_file_dialog;
#[cfg(feature = "directories")]
pub mod save_file_dialog;

pub const MIN_FILE_DIALOG_SIZE: (usize, usize) = (196, 168);

pub trait FileDialogResults<SR: Clone + Debug + PartialEq> {
    fn save_file_result(path: String) -> SR;
    fn load_file_result(path: String) -> SR;
}

/// Creates a shade and outline for dialogs
pub fn dialog_background(width: usize, height: usize, style: &DialogStyle) -> ShapeCollection {
    let mut background = ShapeCollection::default();
    if let Some(color) = style.shade {
        InsertShape::insert_above(
            &mut background,
            Rect::new((0, 0), (width, height)),
            fill(color),
        );
    }
    if let Some(color) = style.background {
        InsertShape::insert_above(&mut background, style.bounds.clone(), fill(color));
    }
    if let Some(color) = style.shadow {
        InsertShape::insert_above(
            &mut background,
            style.bounds.translate_by(coord!(1, 1)),
            stroke(color),
        );
    }
    if let Some(color) = style.border {
        InsertShape::insert_above(&mut background, style.bounds.clone(), stroke(color));
    }
    background
}
