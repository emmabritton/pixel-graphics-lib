use crate::prelude::Rect;
use crate::ui::UiElement;
use std::fmt::Debug;

pub mod column;
pub mod relative;
pub mod row;

pub type ViewId = usize;

pub trait LayoutView: UiElement + Debug {
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
/// `padding` is used to offset views from the relevant position
/// `spacing` is space between views
///
/// **Usage**
/// ```ignore
///  let button1 = Button::new("Button", ...);
///  let button2 = Button::new("Button", ...);
///  let button3 = Button::new("Button", ...);
///  column_layout!(Rect::new(16,16,16,16), ColumnGravity::Left, padding: 2, spacing: 8,  views: button3, button1, button2);
///  // button3 top left will be (18, 18)
///  // button1 top left will be (18, 18 + 8 + button3.height)
///  // button2 top left will be (18, 18 + 8 + button3.height + 8 + button2.height)
/// ```
#[macro_export]
macro_rules! column_layout {
    ($bounds:expr, $gravity:expr, $(padding: $padding:expr,)? $(spacing: $margin:expr,)? views: $($views:expr),+ $(,)?) => {{
        $crate::ui::layout::column::ColumnLayout::new(or_else!($($padding)?, 0), or_else!($($margin)?, 0), $bounds, $gravity).layout(&mut [$(&mut $views,)*])
    }};
}

/// Update the position multiple views to be in a row
/// if `gravity` is
///  - Top - then only the `top_left` of `bounds` is used
///  - Center - then only the `center` of `bounds` is used
///  - Bottom - then only the `bottom_right` of `bounds` is used
/// `padding` is used to offset views from the relevant position
/// `spacing` is space between views
///
/// **Usage**
/// ```ignore
///  let button1 = Button::new("Button", ...);
///  let button2 = Button::new("Button", ...);
///  let button3 = Button::new("Button", ...);
///  row_layout!(Rect::new(16,16,16,16), RowGravity::Top, spacing: 8,  views: button3, button1, button2);
///  // button3 top left will be (16,16)
///  // button1 top left will be (16 + 8 + button3.width, 16)
///  // button2 top left will be (16 + 8 + 8 + button3.width + button1.width, 16)
/// ```
#[macro_export]
macro_rules! row_layout {
    ($bounds:expr, $gravity:expr, $(padding: $padding:expr,)? $(spacing: $margin:expr,)? views: $($views:expr),+ $(,)?) => {{
        $crate::ui::layout::row::RowLayout::new(or_else!($($padding)?, 0), or_else!($($margin)?, 0), $bounds, $gravity).layout(&mut [$(&mut $views,)*])
    }};
}

#[cfg(test)]
mod test {
    use crate::ui::button::Button;
    use crate::ui::prelude::*;
    use crate::ui::UiElement;

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
