use crate::ui::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ColumnGravity {
    Left,
    Center,
    Right,
}

/// Position a collection of views into a column
/// This doesn't act as a container or parent and only moves the views and isn't needed after it's been used
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColumnLayout {
    pub padding: usize,
    pub spacing: usize,
    pub bounds: Rect,
    pub gravity: ColumnGravity,
}

impl ColumnLayout {
    ///# Arguments
    ///* `padding` - Pixel distance to move all views from the topleft or bottomright (depending on gravity)
    ///* `spacing` - Pixel distance to leave between views
    ///* `bounds` - Depends on gravity, if it's
    ///
    /// | Gravity | Effects |
    /// |---------|---------|
    /// |Left| bounds.top_left() is the starting point, bottom and right are ignored|
    /// |Right| bounds.bottom_right() is the starting point, top and left are ignored|
    /// |Center| bounds.top_left() is the starting point, views will be positioned between left and right|
    ///
    ///* `gravity` - Effects how views are positioned, see above
    pub fn new(padding: usize, spacing: usize, bounds: Rect, gravity: ColumnGravity) -> Self {
        Self {
            padding,
            spacing,
            bounds,
            gravity,
        }
    }

    pub fn new_from_topleft(topleft: Coord) -> Self {
        Self::new(
            0,
            0,
            Rect::new_with_size(topleft, 1000000, 100000),
            ColumnGravity::Left,
        )
    }

    pub fn new_bounded(bounds: Rect) -> Self {
        Self::new(0, 0, bounds, ColumnGravity::Left)
    }
}

impl ColumnLayout {
    /// Reposition views in a column
    pub fn layout(&self, views: &mut [&mut dyn PixelView]) {
        let mut y = self.padding;
        for view in views {
            let x = match self.gravity {
                ColumnGravity::Left => self.padding,
                ColumnGravity::Center => {
                    (self.bounds.width() / 2).saturating_sub(view.bounds().width() / 2)
                }
                ColumnGravity::Right => self
                    .bounds
                    .width()
                    .saturating_sub(view.bounds().width())
                    .saturating_sub(self.padding),
            };
            view.set_position(self.bounds.top_left() + (x, y));
            y += view.bounds().height();
            y += self.spacing;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ui::prelude::*;

    #[test]
    fn column_defaults() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Left,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 0));
        assert_eq!(view2.bounds().top_left(), coord!(0, 16));
    }

    #[test]
    fn column_defaults_with_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 10,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Left,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(10, 10));
        assert_eq!(view2.bounds().top_left(), coord!(10, 26));
    }

    #[test]
    fn column_defaults_with_spacing() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 0,
            spacing: 8,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Left,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 0));
        assert_eq!(view2.bounds().top_left(), coord!(0, 24));
    }

    #[test]
    fn column_defaults_with_spacing_and_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 10,
            spacing: 8,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Left,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(10, 10));
        assert_eq!(view2.bounds().top_left(), coord!(10, 34));
    }

    #[test]
    fn column_defaults_with_gravity_right() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Right,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(67, 0));
        assert_eq!(view2.bounds().top_left(), coord!(42, 16));
    }

    #[test]
    fn column_defaults_with_gravity_right_and_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = ColumnLayout {
            padding: 20,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Right,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(47, 20));
        assert_eq!(view2.bounds().top_left(), coord!(22, 36));
    }

    #[test]
    fn column_defaults_with_gravity_center() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let mut layout = ColumnLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: ColumnGravity::Center,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(34, 0));
        assert_eq!(view2.bounds().top_left(), coord!(21, 16));

        layout.padding = 20;
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(34, 20));
        assert_eq!(view2.bounds().top_left(), coord!(21, 36));
    }
}
