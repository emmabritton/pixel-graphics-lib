use std::collections::hash_map::Entry;
use crate::prelude::{ElementState, UiElement};
use crate::ui::layout::*;
use crate::Timing;
use buffer_graphics_lib::Graphics;
use graphics_shapes::coord::Coord;
use graphics_shapes::prelude::Rect;
use crate::ui::button::Button;
use crate::ui::dir_panel::DirResult;

pub enum ClickResult {
    Button(ID),
    TextField(ID),
    DirPanel(ID, Option<DirResult>),
    ToggleButton(ID),
    IconButton(ID),
    IconToggleButton(ID)
}

impl UiCollection {
    pub fn get(&self, element_type: ElementType) -> &[(ID, Box<dyn> {

    }

    pub fn on_mouse_click(&self, mouse_xy: Coord) -> Option<ClickResult> {
        if self.state == ElementState::Disabled {
            return None;
        }

        None
    }
}

impl Layout for UiCollection {
    fn add(&mut self, id: ID, element: Box<dyn UiElement>) -> bool {
        if let Entry::Vacant(e) = self.elements.entry(id) {
            e.insert(element);
            self.ids_order.push(id);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: ID) -> bool {
        if let Some(idx) = self.ids_order.iter().position(|i| i == &id) {
            self.ids_order.remove(idx);
        }
        self.elements.remove(&id).is_some()
    }

    fn relayout(&mut self) {
        match self.positioning {
            Positioning::Absolute => {}
            Positioning::Column => {}
            Positioning::Row => {}
        }
    }
}

impl UiElement for UiCollection {
    #[inline]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        for element in self.elements.values() {
            element.render(graphics, mouse_xy);
        }
    }

    fn update(&mut self, timing: &Timing) {
        for element in self.elements.values_mut() {
            element.update(timing);
        }
    }

    #[inline]
    fn set_state(&mut self, new_state: ElementState) {
        self.state = new_state;
    }

    #[inline]
    fn get_state(&self) -> ElementState {
        self.state
    }
}
