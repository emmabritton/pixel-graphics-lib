use crate::ui::prelude::*;
use buffer_graphics_lib::Graphics;
use rustc_hash::FxHashMap;
use std::hash::Hash;

pub struct ToggleButtonGroup<Key: Hash + Clone + PartialEq + Eq> {
    buttons: FxHashMap<Key, ToggleButton>,
    selected: Key,
}

impl<Key: Hash + Clone + PartialEq + Eq> ToggleButtonGroup<Key> {
    pub fn new(id_button_map: Vec<(Key, ToggleButton)>) -> Self {
        let mut buttons = FxHashMap::default();
        let first = id_button_map.first().unwrap().0.clone();
        for (id, button) in id_button_map.into_iter() {
            buttons.insert(id.clone(), button);
        }
        let mut group = Self {
            buttons,
            selected: first.clone(),
        };
        group.set_selected(first);
        group
    }
}

impl<Key: Hash + Clone + PartialEq + Eq> ToggleButtonGroup<Key> {
    pub fn on_mouse_click(&mut self, mouse_xy: Coord) -> Option<Key> {
        let mut tmp = None;
        for (idx, button) in &mut self.buttons {
            if button.on_mouse_click(mouse_xy) {
                tmp = Some(idx.clone());
            }
        }
        if let Some(id) = &tmp {
            for (idx, button) in &mut self.buttons {
                button.set_selected(idx == id);
            }
        }
        tmp
    }

    pub fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        for button in self.buttons.values() {
            button.render(graphics, mouse_xy);
        }
    }

    pub fn get_selected(&self) -> &Key {
        &self.selected
    }

    pub fn set_selected(&mut self, id: Key) {
        for (idx, button) in &mut self.buttons {
            button.set_selected(idx == &id);
        }
        self.selected = id;
    }
}

pub struct ToggleIconButtonGroup<Key: Hash + Clone + PartialEq + Eq> {
    buttons: FxHashMap<Key, ToggleIconButton>,
    selected: Key,
}

impl<Key: Hash + Clone + PartialEq + Eq> ToggleIconButtonGroup<Key> {
    pub fn new(id_button_map: Vec<(Key, ToggleIconButton)>) -> Self {
        let mut buttons = FxHashMap::default();
        let first = id_button_map.first().unwrap().0.clone();
        for (id, button) in id_button_map.into_iter() {
            buttons.insert(id.clone(), button);
        }
        let mut group = Self {
            buttons,
            selected: first.clone(),
        };
        group.set_selected(first);
        group
    }
}

impl<Key: Hash + Clone + PartialEq + Eq> ToggleIconButtonGroup<Key> {
    pub fn on_mouse_click(&mut self, mouse_xy: Coord) -> Option<Key> {
        let mut tmp = None;
        for (idx, button) in &mut self.buttons {
            if button.on_mouse_click(mouse_xy) {
                tmp = Some(idx.clone());
            }
        }
        if let Some(id) = &tmp {
            for (idx, button) in &mut self.buttons {
                button.set_selected(idx == id);
            }
        }
        tmp
    }

    pub fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        for button in self.buttons.values() {
            button.render(graphics, mouse_xy);
        }
    }

    pub fn get_selected(&self) -> &Key {
        &self.selected
    }

    pub fn set_selected(&mut self, id: Key) {
        for (idx, button) in &mut self.buttons {
            button.set_selected(idx == &id);
        }
        self.selected = id;
    }
}
