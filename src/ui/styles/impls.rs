use crate::ui::prelude::*;
use buffer_graphics_lib::prelude::*;

impl ColorSet {
    pub fn new(
        normal: Option<Color>,
        hover: Option<Color>,
        error: Option<Color>,
        disabled: Option<Color>,
    ) -> Self {
        Self {
            normal,
            hover,
            error,
            disabled,
        }
    }

    pub fn new_values(normal: Color, hover: Color, error: Color, disabled: Color) -> Self {
        Self::new(Some(normal), Some(hover), Some(error), Some(disabled))
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(Some(color), Some(color), Some(color), Some(color))
    }

    pub fn get(&self, hovering: bool, error: bool, disabled: bool) -> Option<Color> {
        if disabled {
            self.disabled
        } else if error {
            self.error
        } else if hovering {
            self.hover
        } else {
            self.normal
        }
    }
}

impl FocusColorSet {
    pub fn new(
        normal: Option<Color>,
        hover: Option<Color>,
        focused: Option<Color>,
        error: Option<Color>,
        disabled: Option<Color>,
    ) -> Self {
        Self {
            normal,
            hover,
            focused,
            error,
            disabled,
        }
    }

    pub fn new_values(
        normal: Color,
        hover: Color,
        focused: Color,
        error: Color,
        disabled: Color,
    ) -> Self {
        Self::new(
            Some(normal),
            Some(hover),
            Some(focused),
            Some(error),
            Some(disabled),
        )
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(
            Some(color),
            Some(color),
            Some(color),
            Some(color),
            Some(color),
        )
    }

    pub fn get(&self, hovering: bool, focused: bool, error: bool, disabled: bool) -> Option<Color> {
        if disabled {
            self.disabled
        } else if error {
            self.error
        } else if focused {
            self.focused
        } else if hovering {
            self.hover
        } else {
            self.normal
        }
    }
}

impl ToggleColorSet {
    pub fn new(
        normal: Option<Color>,
        hover: Option<Color>,
        toggled: Option<Color>,
        hover_toggled: Option<Color>,
        error: Option<Color>,
        disabled: Option<Color>,
    ) -> Self {
        Self {
            normal,
            hover,
            toggled,
            hover_toggled,
            error,
            disabled,
        }
    }

    pub fn new_values(
        normal: Color,
        hover: Color,
        toggled: Color,
        hover_toggled: Color,
        error: Color,
        disabled: Color,
    ) -> Self {
        Self::new(
            Some(normal),
            Some(hover),
            Some(toggled),
            Some(hover_toggled),
            Some(error),
            Some(disabled),
        )
    }

    pub fn new_same_hover(
        normal: Option<Color>,
        hover: Option<Color>,
        toggled: Option<Color>,
        error: Option<Color>,
        disabled: Option<Color>,
    ) -> Self {
        Self::new(normal, hover, toggled, hover, error, disabled)
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(
            Some(color),
            Some(color),
            Some(color),
            Some(color),
            Some(color),
            Some(color),
        )
    }
    pub fn get(&self, hovering: bool, toggled: bool, error: bool, disabled: bool) -> Option<Color> {
        if disabled {
            self.disabled
        } else if error {
            self.error
        } else {
            match (hovering, toggled) {
                (true, true) => self.hover_toggled,
                (true, false) => self.hover,
                (false, true) => self.toggled,
                (false, false) => self.normal,
            }
        }
    }
}
