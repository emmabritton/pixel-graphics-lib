pub mod alert;
pub mod button;
pub mod dir_panel;
pub mod helpers;
pub mod icon_button;
pub mod label;
pub mod layout;
pub mod menu_bar;
mod menu_item_view;
pub mod styles;
pub mod text_field;
pub mod toggle_button;
pub mod toggle_icon_button;
pub mod tooltip;

use crate::prelude::*;
use buffer_graphics_lib::prelude::*;
use std::fmt::Debug;

pub mod prelude {
    pub use crate::ui::alert::*;
    pub use crate::ui::button::*;
    pub use crate::ui::dir_panel::*;
    pub use crate::ui::helpers::*;
    pub use crate::ui::icon_button::*;
    pub use crate::ui::label::*;
    pub use crate::ui::layout::column::*;
    pub use crate::ui::layout::row::*;
    pub use crate::ui::layout::*;
    pub use crate::ui::menu_bar::*;
    pub use crate::ui::styles::*;
    pub use crate::ui::text_field::*;
    pub use crate::ui::toggle_button::*;
    pub use crate::ui::toggle_icon_button::*;
    pub use crate::ui::tooltip::*;
    pub use crate::ui::*;
}

#[macro_export]
macro_rules! render {
    ($graphics:tt, $mouse:tt, $($view:expr$(,)?)*) => {
        $(
            $view.render($graphics, $mouse);
        )*
    };
}

pub trait PixelView {
    fn set_position(&mut self, top_left: Coord);

    fn bounds(&self) -> &Rect;

    fn render(&self, graphics: &mut Graphics, mouse: &MouseData);

    fn update(&mut self, timing: &Timing);

    fn set_state(&mut self, new_state: ViewState);

    fn get_state(&self) -> ViewState;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ViewState {
    Normal,
    Disabled,
    Error,
}

impl ViewState {
    /// Return pair of (is_error, is_disabled)
    pub fn get_err_dis(&self) -> (bool, bool) {
        match self {
            ViewState::Normal => (false, false),
            ViewState::Disabled => (false, true),
            ViewState::Error => (true, false),
        }
    }
}
