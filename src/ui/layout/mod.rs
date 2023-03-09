use buffer_graphics_lib::Graphics;
use graphics_shapes::coord::Coord;
use graphics_shapes::prelude::Rect;
use winit::event::VirtualKeyCode;
use crate::prelude::{ElementState, UiElement};
use crate::Timing;

trait Layout: UiElement {
    fn on_mouse_click(&self, mouse_xy: Coord) -> Option<ID>;

    fn get(&self, id: ID) -> Option<&Box<dyn UiElement>>;

    fn get_mut(&mut self, id: ID) -> Option<&mut Box<dyn UiElement>>;

    fn add(&mut self, id: ID, element: Box<dyn UiElement>) -> bool;

    fn remove(&mut self, id: ID) -> bool;

    fn relayout(&mut self);
}

struct Column {
    ui: UiCollection
}

impl UiElement for Column {
    fn bounds(&self) -> &Rect {
        self.ui.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        self.ui.render(graphics, mouse_xy);
    }

    fn update(&mut self, timing: &Timing) {
        todo!()
    }

    fn on_mouse_click(&mut self, mouse_xy: Coord) -> bool {
        todo!()
    }

    fn on_key_press(&mut self, keys: &[VirtualKeyCode]) {
        todo!()
    }

    fn set_state(&mut self, new_state: ElementState) {
        todo!()
    }

    fn get_state(&self) -> ElementState {
        todo!()
    }
}