use crate::prelude::UiElement;
use crate::ui::ElementState;
use graphics_shapes::coord::Coord;
use graphics_shapes::prelude::Rect;
use rustc_hash::FxHashMap;

pub mod collection;
pub mod column;
pub mod row;

type ID = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ElementType {
    Button,
    IconButton,
    ToggleButton,
    ToggleIconButton,
    TextField,
    DirPanel
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Positioning {
    Absolute,
    Column,
    Row,
}

struct UiCollection {
    elements: FxHashMap<ID, Box<dyn UiElement>>,
    bounds: Rect,
    positioning: Positioning,
    ids_order: Vec<ID>,
    state: ElementState,
}

pub trait Layout: UiElement {
    fn add(&mut self, id: ID, element: Box<dyn UiElement>) -> bool;

    fn remove(&mut self, id: ID) -> bool;

    fn relayout(&mut self);
}
