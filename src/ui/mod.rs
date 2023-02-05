pub mod alert;
pub mod button;
pub mod dir_panel;
pub mod styles;
pub mod text_field;
pub mod toggle_button;

use crate::prelude::*;
use std::fmt::Debug;

pub mod prelude {
    pub use crate::ui::alert::*;
    pub use crate::ui::button::*;
    pub use crate::ui::dir_panel::*;
    pub use crate::ui::styles::*;
    pub use crate::ui::text_field::*;
    pub use crate::ui::toggle_button::*;
    pub use crate::ui::*;
}

pub trait Ui: Debug {
    fn bounds(&self) -> &Rect;

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord);
}
