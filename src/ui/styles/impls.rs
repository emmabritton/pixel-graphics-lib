use crate::prelude::styles::*;

impl ColorSet {
    pub fn new(normal: Option<Color>, hover: Option<Color>) -> Self {
        Self { normal, hover }
    }

    pub fn new_values(normal: Color, hover: Color) -> Self {
        Self::new(Some(normal), Some(hover))
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(Some(color), Some(color))
    }

    pub fn get(&self, hovering: bool) -> Option<Color> {
        if hovering {
            self.hover
        } else {
            self.normal
        }
    }
}

impl FocusColorSet {
    pub fn new(normal: Option<Color>, hover: Option<Color>, focused: Option<Color>) -> Self {
        Self {
            normal,
            hover,
            focused,
        }
    }

    pub fn new_values(normal: Color, hover: Color, focused: Color) -> Self {
        Self::new(Some(normal), Some(hover), Some(focused))
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(Some(color), Some(color), Some(color))
    }

    pub fn get(&self, hovering: bool, focused: bool) -> Option<Color> {
        if focused {
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
    ) -> Self {
        Self {
            normal,
            hover,
            toggled,
            hover_toggled,
        }
    }

    pub fn new_values(normal: Color, hover: Color, toggled: Color, hover_toggled: Color) -> Self {
        Self::new(
            Some(normal),
            Some(hover),
            Some(toggled),
            Some(hover_toggled),
        )
    }

    pub fn new_same_hover(
        normal: Option<Color>,
        hover: Option<Color>,
        toggled: Option<Color>,
    ) -> Self {
        Self::new(normal, hover, toggled, hover)
    }

    pub fn new_same(color: Color) -> Self {
        Self::new(Some(color), Some(color), Some(color), Some(color))
    }

    pub fn get(&self, hovering: bool, toggled: bool) -> Option<Color> {
        match (hovering, toggled) {
            (true, true) => self.hover_toggled,
            (true, false) => self.hover,
            (false, true) => self.toggled,
            (false, false) => self.normal,
        }
    }
}
