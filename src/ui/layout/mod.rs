//! UI Layout
//!
//! There's three main approaches for positioning UI
//! 1) Absolute:
//!     Each view is positioned manually at specific coords
//! 2) Assisted
//!     Using the [RowLayout] and [ColumnLayout] macros views are positioned in rows and columns (with spacing and padding)
//! 3) Relative
//!     Using [LayoutContext] and [layout!](crate::ui::layout::relative::layout) to position and size views relative to the context and each other
//!
//! # Examples
//!
//! ### 1 Absolute
//! ```rust
//!# use pixels_graphics_lib::prelude::*;
//!# use pixels_graphics_lib::ui::prelude::*;
//!# let style = UiStyle::default();
//! const PADDING: usize = 4;
//! const BUTTON_HEIGHT: usize = 30;
//! const BUTTON_WIDTH: usize = 70;
//! const HEIGHT: usize = 100;
//! const WIDTH: usize = 100;
//! let background = Rect::new((0,0), (WIDTH, HEIGHT));
//! let label = Text::new("Are you sure?", TextPos::px((100, 50)), (WHITE, PixelFont::Standard6x7, Positioning::Center));
//! let positive = Button::new((PADDING, HEIGHT - PADDING - BUTTON_HEIGHT), "Yes", Some(BUTTON_WIDTH), &style.button);
//! let negative = Button::new((WIDTH - PADDING - BUTTON_WIDTH, HEIGHT - PADDING - BUTTON_HEIGHT), "No", Some(BUTTON_WIDTH), &style.button);
//! ```
//!
//! ### 2 Assisted
//!
//!```rust
//!# use buffer_graphics_lib::text::PixelFont::Standard6x7;
//!# use pixels_graphics_lib::column_layout;
//!# use pixels_graphics_lib::ui::prelude::*;
//!# use pixels_graphics_lib::prelude::*;
//!# let style = UiStyle::default();
//! let mut lbl_name = Label::new(Text::new("Name", TextPos::px(Coord::default()), (WHITE, Standard6x7)));
//! let mut txt_name = TextField::new(Coord::default(), 30, Standard6x7, (None, None), "", &[TextFilter::Letters], &style.text_field);
//! let mut cta = Button::new(Coord::default(), "Submit", None, &style.button);
//! column_layout!(Rect::new((0,0),(200,200)), ColumnGravity::Left, padding: 4, views: lbl_name, txt_name, cta);
//! ```
//!
//! ### 3 Relative
//! ```rust
//! use pixels_graphics_lib::layout;
//!# use pixels_graphics_lib::prelude::*;
//!# use pixels_graphics_lib::ui::layout::relative::LayoutContext;
//!# use pixels_graphics_lib::ui::prelude::*;
//!# let style = UiStyle::default();
//! const BUTTON_WIDTH: usize = 70;
//! const PADDING: usize = 6;
//! const HEIGHT: usize = 100;
//! const WIDTH: usize = 100;
//! let background = Rect::new((0,0), (WIDTH, HEIGHT));
//! let context = LayoutContext::new_with_padding(background.clone(), PADDING);
//! let mut label = Label::new(Text::new("Are you sure?", TextPos::px((100, 50)), (WHITE, PixelFont::Standard6x7, Positioning::Center)));
//! let mut positive = Button::new((0,0), "Yes", Some(BUTTON_WIDTH), &style.button);
//! let mut negative = Button::new((0,0), "No", Some(BUTTON_WIDTH), &style.button);
//!
//! layout!(context, label, align_centerh);
//! layout!(context, label, align_top);
//!
//! layout!(context, positive, align_left);
//! layout!(context, positive, align_bottom);
//!
//! layout!(context, negative, align_left);
//! layout!(context, negative, align_bottom);
//! ```

use crate::prelude::Rect;
use crate::ui::PixelView;
use std::fmt::Debug;

pub mod column;
pub mod relative;
pub mod row;

pub type ViewId = usize;

pub trait LayoutView: PixelView + Debug {
    fn set_bounds(&mut self, bounds: Rect);
}

#[macro_export]
macro_rules! or_else {
    ($value:expr, $other: expr) => {
        $value
    };
    (, $other: expr) => {
        $other
    };
}

#[macro_export]
macro_rules! bounds {
    ($top_left:expr, $bottom_right:expr) => {
        Rect::new(
            $crate::prelude::coord!($top_left),
            $crate::prelude::coord!($bottom_right),
        )
    };
    ($top_left:expr, $width: expr, $height: expr) => {
        Rect::new_with_size(
            $crate::prelude::coord!($top_left),
            $width as usize,
            $height as usize,
        )
    };
}

/// Update the position multiple views to be in a column
/// if `gravity` is
///  - Left - then only the `top_left` of `bounds` is used
///  - Center - then only the `center` of `bounds` is used
///  - Right - then only the `bottom_right` of `bounds` is used
///
/// `padding` is used to offset views from the relevant position
///
/// `spacing` is space between views
///
/// **Usage**
/// ```ignore
///  let button1 = Button::new("Button", ...);
///  let button2 = Button::new("Button", ...);
///  let button3 = Button::new("Button", ...);
///  column_layout!(Rect::new((16,16),(16,16)), ColumnGravity::Left, padding: 2, spacing: 8,  views: button3, button1, button2);
///  // button3 top left will be (18, 18)
///  // button1 top left will be (18, 18 + 8 + button3.height)
///  // button2 top left will be (18, 18 + 8 + button3.height + 8 + button2.height)
/// ```
#[macro_export]
macro_rules! column_layout {
    ($bounds:expr, $gravity:expr, $(padding: $padding:expr,)? $(spacing: $margin:expr,)? views: $($views:expr),+ $(,)?) => {{
        $crate::ui::layout::column::ColumnLayout::new($crate::or_else!($($padding)?, 0),$crate::or_else!($($margin)?, 0), $bounds, $gravity).layout(&mut [$(&mut $views,)*])
    }};
}

/// Update the position multiple views to be in a row
/// if `gravity` is
///  - Top - then only the `top_left` of `bounds` is used
///  - Center - then only the `center` of `bounds` is used
///  - Bottom - then only the `bottom_right` of `bounds` is used
///
/// `padding` is used to offset views from the relevant position
///
/// `spacing` is space between views
///
/// **Usage**
/// ```ignore
///  let button1 = Button::new("Button", ...);
///  let button2 = Button::new("Button", ...);
///  let button3 = Button::new("Button", ...);
///  row_layout!(Rect::new((16,16),(16,16)), RowGravity::Top, spacing: 8,  views: button3, button1, button2);
///  // button3 top left will be (16,16)
///  // button1 top left will be (16 + 8 + button3.width, 16)
///  // button2 top left will be (16 + 8 + 8 + button3.width + button1.width, 16)
/// ```
#[macro_export]
macro_rules! row_layout {
    ($bounds:expr, $gravity:expr, $(padding: $padding:expr,)? $(spacing: $margin:expr,)? views: $($views:expr),+ $(,)?) => {{
        $crate::ui::layout::row::RowLayout::new($crate::or_else!($($padding)?, 0), $crate::or_else!($($margin)?, 0), $bounds, $gravity).layout(&mut [$(&mut $views,)*])
    }};
}

#[cfg(test)]
mod test {
    use crate::ui::button::Button;
    use crate::ui::prelude::*;
    use crate::ui::PixelView;

    #[test]
    fn syntax_check() {
        let style = UiStyle::default();
        let mut button = Button::new((0, 0), "test", None, &style.button);
        let mut button2 = Button::new((0, 0), "test", None, &style.button);
        column_layout!(
            bounds!((100, 100), 10, 10),
            ColumnGravity::Left,
            views: button
        );
        assert_eq!(button.bounds().top_left(), coord!(100, 100));

        column_layout!(
            bounds!((100, 100), 10, 10),
            ColumnGravity::Right,
            padding: 4,
            views: button
        );
        assert_eq!(button.bounds().top_left(), coord!(100, 104));

        row_layout!(
            bounds!((100, 100), 10, 10),
            RowGravity::Center,
            spacing: 4,
            views: button, button2
        );
        assert_eq!(button.bounds().top_left(), coord!(100, 100));
        assert_eq!(button2.bounds().top_left(), coord!(137, 100));
    }

    #[test]
    fn bounds_check() {
        let bounds = bounds!((0, 0), 100, 100);
        assert_eq!(bounds, Rect::new_with_size(Coord::new(0, 0), 100, 100));

        let bounds = bounds!((45, 47), (100, 100));
        assert_eq!(bounds, Rect::new(Coord::new(45, 47), Coord::new(100, 100)));

        let bounds = bounds!((45, 47), 100_isize, 100_i32);
        assert_eq!(bounds, Rect::new(Coord::new(45, 47), Coord::new(145, 147)));
    }
}
