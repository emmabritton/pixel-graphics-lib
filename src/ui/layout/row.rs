use crate::ui::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RowGravity {
    Top,
    Center,
    Bottom,
}

/// Position a collection of views into a row
/// This doesn't act as a container or parent and only moves the views and isn't needed after it's been used
#[derive(Debug, Clone)]
pub struct RowLayout {
    pub padding: usize,
    pub spacing: usize,
    pub bounds: Rect,
    pub gravity: RowGravity,
}

impl RowLayout {
    ///# Arguments
    ///* `padding` - Pixel distance to move all views from the topleft or bottomright (depending on gravity)
    ///* `spacing` - Pixel distance to leave between views
    ///* `bounds` - Depends on gravity, if it's
    ///
    /// | Gravity | Effects |
    /// |---------|---------|
    /// |Top| bounds.top_left() is the starting point, bottom and right are ignored|
    /// |Bottom| bounds.bottom_right() is the starting point, top and left are ignored|
    /// |Center| bounds.top_left() is the starting point, views will be positioned between top and bottom|
    ///
    ///* `gravity` - Effects how views are positioned, see above
    pub fn new(padding: usize, spacing: usize, bounds: Rect, gravity: RowGravity) -> Self {
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
            RowGravity::Top,
        )
    }

    pub fn new_bounded(bounds: Rect) -> Self {
        Self::new(0, 0, bounds, RowGravity::Top)
    }
}

impl RowLayout {
    /// Reposition views in a row
    pub fn layout(&self, views: &mut [&mut dyn UiElement]) {
        let mut x = self.padding;
        for view in views {
            let y = match self.gravity {
                RowGravity::Top => self.padding,
                RowGravity::Center => {
                    (self.bounds.height() / 2).saturating_sub(view.bounds().height() / 2)
                }
                RowGravity::Bottom => (self.bounds.height())
                    .saturating_sub(view.bounds().height())
                    .saturating_sub(self.padding),
            };
            view.set_position(self.bounds.top_left() + (x, y));
            x += view.bounds().width();
            x += self.spacing;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ui::prelude::*;

    #[test]
    fn row_defaults() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Top,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 0));
        assert_eq!(view2.bounds().top_left(), coord!(33, 0));
    }

    #[test]
    fn row_defaults_with_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 10,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Top,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(10, 10));
        assert_eq!(view2.bounds().top_left(), coord!(43, 10));
    }

    #[test]
    fn row_defaults_with_spacing() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 0,
            spacing: 8,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Top,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 0));
        assert_eq!(view2.bounds().top_left(), coord!(41, 0));
    }

    #[test]
    fn row_defaults_with_spacing_and_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 10,
            spacing: 8,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Top,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(10, 10));
        assert_eq!(view2.bounds().top_left(), coord!(51, 10));
    }

    #[test]
    fn row_defaults_with_gravity_right() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Bottom,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 84));
        assert_eq!(view2.bounds().top_left(), coord!(33, 84));
    }

    #[test]
    fn row_defaults_with_gravity_right_and_padding() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let layout = RowLayout {
            padding: 20,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Bottom,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(20, 64));
        assert_eq!(view2.bounds().top_left(), coord!(53, 64));
    }

    #[test]
    fn row_defaults_with_gravity_center() {
        let style = UiStyle::default();
        let mut view1 = Button::new((0, 0), "Test", None, &style.button);
        let mut view2 = Button::new((0, 0), "Another", None, &style.button);
        let mut layout = RowLayout {
            padding: 0,
            spacing: 0,
            bounds: Rect::new((0, 0), (100, 100)),
            gravity: RowGravity::Center,
        };
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(0, 42));
        assert_eq!(view2.bounds().top_left(), coord!(33, 42));

        layout.padding = 20;
        layout.layout(&mut [&mut view1, &mut view2]);

        assert_eq!(view1.bounds().top_left(), coord!(20, 42));
        assert_eq!(view2.bounds().top_left(), coord!(53, 42));
    }
}
