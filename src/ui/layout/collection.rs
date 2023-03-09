use buffer_graphics_lib::Graphics;
use graphics_shapes::coord::Coord;
use graphics_shapes::prelude::Rect;
use rustc_hash::FxHashMap;
use winit::event::VirtualKeyCode;
use crate::prelude::{ElementState, UiElement};
use crate::Timing;

type ID = u64;

struct UiCollection {
    elements: FxHashMap<ID, Box<dyn UiElement>>,
    bounds: Rect
}

impl Layout for UiCollection {
    fn on_mouse_click(&self, mouse_xy: Coord) -> Option<ID> {
        for (id, mut element) in self.elements {
            if element.on_mouse_click(mouse_xy) {
                return Some(id)
            }
        }
        None
    }

    fn get(&self, id: ID) -> Option<&Box<dyn UiElement>> {
        self.elements.get(&id)
    }

    fn get_mut(&mut self, id: ID) -> Option<&mut Box<dyn UiElement>> {
        self.elements.get_mut(&id)
    }

    fn add(&mut self, id: ID, element: Box<dyn UiElement>) -> bool {
        if self.elements.contains_key(&id) {
            false
        } else {
            self.elements.insert(id, element);
            true
        }
    }

    fn remove(&mut self, id: ID) -> bool {
        self.elements.remove(&id).is_some()
    }
}

impl UiElement for UiCollection {
    #[inline]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        for (_, element) in self.elements {
            element.render(graphics, mouse_xy);
        }
    }

    fn update(&mut self, timing: &Timing) {
        for (_, mut element) in self.elements {
            element.update(timing);
        }
    }

    fn on_mouse_click(&self, _: Coord) -> bool {
        todo!("Do not use?")
    }

    fn on_key_press(&self, keys: &[VirtualKeyCode]) {
        for (_, mut element) in self.elements {
            element.on_key_press(keys);
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