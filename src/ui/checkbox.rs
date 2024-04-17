use crate::prelude::*;
use crate::ui::layout::LayoutView;
use crate::ui::prelude::CheckboxStyle;
use crate::ui::{PixelView, ViewState};

#[derive(Debug)]
pub struct Checkbox {
    bounds: Rect,
    text: Text,
    checked: bool,
    style: CheckboxStyle,
    state: ViewState,
}

impl Checkbox {
    pub fn new<P: Into<Coord>>(pos: P, text: &str, checked: bool, style: &CheckboxStyle) -> Self {
        let pos = pos.into();
        let (w, h) = style.font.measure(text);
        let w = w + style.spacing + style.check_box.width() as usize;
        let bounds = Rect::new_with_size(pos, w, h);
        let text = Checkbox::layout(style, bounds.clone(), text);
        Self {
            bounds,
            text,
            checked,
            style: style.clone(),
            state: ViewState::Normal,
        }
    }

    fn text(&self) -> String {
        self.text
            .contents()
            .iter()
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn layout(style: &CheckboxStyle, bounds: Rect, text: &str) -> Text {
        let text_width = bounds
            .width()
            .saturating_sub(style.check_box.width() as usize);
        let pos = bounds.top_left()
            + (
                style.check_box.width() as usize + style.spacing,
                bounds.height() / 2,
            );
        Text::new(
            text,
            TextPos::px(pos),
            (
                TRANSPARENT,
                style.font,
                WrappingStrategy::SpaceBeforeCol(style.font.px_to_cols(text_width)),
                Positioning::LeftCenter,
            ),
        )
    }
}

impl Checkbox {
    /// # Returns
    /// * `None` - Click was outside view or view is disabled
    /// * `Some(bool)` - new checked state
    #[must_use]
    pub fn on_mouse_click(&mut self, down: Coord, up: Coord) -> Option<bool> {
        if self.state != ViewState::Disabled {
            if self.bounds.contains(down) && self.bounds.contains(up) {
                self.checked = !self.checked;
                Some(self.checked)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}

impl PixelView for Checkbox {
    fn set_position(&mut self, top_left: Coord) {
        self.set_bounds(self.bounds.move_to(top_left));
    }

    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        let hover = self.bounds.contains(mouse.xy);
        let (err, dir) = self.state.get_err_dis();
        graphics.draw_indexed_image(self.bounds.top_left(), &self.style.check_box);
        if self.checked {
            graphics.draw_indexed_image(self.bounds.top_left(), &self.style.checked_icon);
        }
        if let Some(color) = self.style.text.get(hover, err, dir) {
            self.text.with_color(color).render(graphics);
        }
    }

    fn update(&mut self, _: &Timing) {}

    fn set_state(&mut self, new_state: ViewState) {
        self.state = new_state;
    }

    fn get_state(&self) -> ViewState {
        self.state
    }
}

impl LayoutView for Checkbox {
    fn set_bounds(&mut self, bounds: Rect) {
        let text = Checkbox::layout(
            &self.style,
            bounds
                .clone()
                .translate_by(coord!(0, self.text.formatting().font().spacing())),
            &self.text(),
        );
        self.bounds = bounds;
        self.text = text;
    }
}
