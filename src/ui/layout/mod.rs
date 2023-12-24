pub mod column;
pub mod row;

#[macro_export]
macro_rules! or_else {
    ($value:literal, $other: literal) => {
        $value
    };
    (, $other: literal) => {
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

#[macro_export]
macro_rules! column_layout {
    ($bounds:expr, $gravity:expr, $(padding: $padding:expr,)? $(spacing: $margin:expr,)? views: $($views:expr),+ $(,)?) => {{
        $crate::ui::layout::column::ColumnLayout::new(or_else!($($padding)?, 0), or_else!($($margin)?, 0), $bounds, $gravity).layout(&mut [$(&mut $views,)*])
    }};
}

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
