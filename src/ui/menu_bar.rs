use crate::ui::menu_item_view::*;
use crate::ui::prelude::*;
use std::hash::Hash;

#[derive(Debug)]
pub struct MenuBar<Key: Hash + Copy + PartialEq + Eq + Debug> {
    bounds: Rect,
    full_bounds: Rect,
    style: MenuBarStyle,
    items: Vec<MenuItemView<Key>>,
    state: ViewState,
    fill_width: bool,
    screen_size: (usize, usize),
}

#[derive(Debug)]
pub struct MenuBarItem<Key: Hash + Copy + PartialEq + Eq + Debug> {
    pub(crate) id: Key,
    pub(crate) name: String,
    pub(crate) children: Option<Vec<MenuBarItem<Key>>>,
    pub(crate) checkable: Option<bool>,
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> MenuBarItem<Key> {
    /// Menu item that can be clicked
    pub fn new_button(id: Key, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            children: None,
            checkable: None,
        }
    }

    /// Menu item that can be toggled on or off
    /// use [MenuBar::is_checked] to check state
    pub fn new_checkable(id: Key, name: &str, default: bool) -> Self {
        Self {
            id,
            name: name.to_string(),
            checkable: Some(default),
            children: None,
        }
    }

    pub fn new_options(id: Key, name: &str, children: &[(Key, &str)], default: usize) -> Self {
        let default = if default >= children.len() {
            0
        } else {
            default
        };
        Self {
            id,
            name: name.to_string(),
            children: Some(
                children
                    .iter()
                    .enumerate()
                    .map(|(i, p)| MenuBarItem::new_checkable(p.0, p.1, i == default))
                    .collect(),
            ),
            checkable: None,
        }
    }

    /// Menu item with children that all act like buttons
    pub fn new_menu(id: Key, name: &str, children: &[(Key, &str)]) -> Self {
        Self {
            id,
            name: name.to_string(),
            children: Some(
                children
                    .iter()
                    .map(|p| MenuBarItem::new_button(p.0, p.1))
                    .collect(),
            ),
            checkable: None,
        }
    }

    pub fn new(id: Key, name: &str, children: Vec<MenuBarItem<Key>>) -> Self {
        Self {
            id,
            name: name.to_string(),
            children: Some(children),
            checkable: None,
        }
    }
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> MenuBar<Key> {
    pub fn new(
        style: &MenuBarStyle,
        pos: Coord,
        screen_size: (usize, usize),
        fill_width: bool,
        items: &[MenuBarItem<Key>],
    ) -> MenuBar<Key> {
        let views: Vec<MenuItemView<Key>> = items
            .iter()
            .map(|item| MenuItemView::new(item, true))
            .collect();
        let bounds = Rect::new(pos, (0, 0));
        let mut menu_bar = MenuBar {
            full_bounds: bounds.clone(),
            bounds,
            style: style.clone(),
            items: views,
            state: ViewState::Normal,
            fill_width,
            screen_size,
        };
        menu_bar.layout();
        menu_bar
    }

    fn layout(&mut self) {
        self.bounds = layout_titles(
            self.bounds.top_left(),
            &self.style,
            self.screen_size,
            self.fill_width,
            &mut self.items,
        );

        self.calc_bounds();
    }
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> MenuBar<Key> {
    pub fn full_bounds(&self) -> &Rect {
        &self.full_bounds
    }

    pub fn on_mouse_move(&mut self, xy: Coord) {
        if self.full_bounds.contains(xy) {
            self.items.iter_mut().for_each(|c| (*c).on_mouse_move(xy));
        }
        self.calc_bounds();
    }

    /// Returns the path when clicked, if it has no children and is state is normal
    ///
    /// Also collapses the menu when clicking on a clickable item or outside the menu
    ///
    /// # Returns
    /// * `None` - outside menu
    /// * one element - on menu bar title item
    /// * two or more elements - on dropdown item
    pub fn on_mouse_click(&mut self, down_at: Coord, up_at: Coord) -> Option<Key> {
        if !self.full_bounds.contains(up_at) {
            self.collapse();
            return None;
        }
        let path = on_click_path(&self.items, down_at, up_at);
        self.collapse();
        path
    }

    pub fn is_expanded(&self) -> bool {
        self.items.iter().any(|v| v.focused)
    }

    /// Get the text label for a menu item, returns None for invalid IDs
    pub fn label_for(&self, id: Key) -> Option<&str> {
        self.get_view(id).map(|v| v.name.as_str())
    }

    /// Set state for a menu item, does nothing for invalid IDs
    /// If a parent is disabled then it's children won't be rendered or clickable but
    /// they have their own state
    pub fn set_state(&mut self, id: Key, new_state: ViewState) {
        if let Some(v) = self.get_view_mut(id) {
            v.state = new_state;
            if new_state == ViewState::Disabled {
                v.focused = false;
            }
        }
        self.calc_bounds();
    }

    /// Get state for a menu item, returns None for invalid IDs
    /// If a parent is disabled then it's children won't be rendered or clickable but
    /// they have their own state
    pub fn get_state(&self, id: Key) -> Option<ViewState> {
        self.get_view(id).map(|v| v.state)
    }

    /// Returns None if ID is invalid or view isn't checkable
    pub fn is_checked(&self, id: Key) -> Option<bool> {
        self.get_view(id).and_then(|v| {
            if let ItemContent::Checkable(checked) = v.content {
                Some(checked)
            } else {
                None
            }
        })
    }

    /// Set checked for a menu item, does nothing for invalid paths
    /// If changing an options group then only `value == true` works
    pub fn set_checked(&mut self, id: Key, value: bool) {
        if let Some(view) = self.get_view_mut(id) {
            if let ItemContent::Checkable(checked) = &mut view.content {
                *checked = value;
            }
        }
    }

    /// Unchecks all checkable direct children
    pub fn uncheck_all_children(&mut self, id: Key) {
        if let Some(view) = self.get_view_mut(id) {
            if let ItemContent::Parent(children, _, _) = &mut view.content {
                for child in children {
                    if matches!(child.content, ItemContent::Checkable(_)) {
                        child.content = ItemContent::Checkable(false);
                    }
                }
            }
        }
    }

    fn get_view(&self, id: Key) -> Option<&MenuItemView<Key>> {
        Self::get_view_from_list(&self.items, id)
    }

    fn get_view_from_list(list: &[MenuItemView<Key>], id: Key) -> Option<&MenuItemView<Key>> {
        for item in list {
            if item.id == id {
                return Some(item);
            } else if let ItemContent::Parent(children, _, _) = &item.content {
                let result = Self::get_view_from_list(children, id);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn get_view_mut(&mut self, id: Key) -> Option<&mut MenuItemView<Key>> {
        Self::get_view_mut_from_list(&mut self.items, id)
    }

    fn get_view_mut_from_list(
        list: &mut [MenuItemView<Key>],
        id: Key,
    ) -> Option<&mut MenuItemView<Key>> {
        for item in list {
            if item.id == id {
                return Some(item);
            } else if let ItemContent::Parent(children, _, _) = &mut item.content {
                let result = Self::get_view_mut_from_list(children, id);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn calc_bounds(&mut self) {
        self.full_bounds = self.bounds.clone();
        for children in &self.items {
            if let Some(extra) = focused_bounds(children) {
                self.full_bounds = union(&self.full_bounds, &extra);
            }
        }
    }

    pub fn collapse(&mut self) {
        collapse_menu(&mut self.items);
    }
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> PixelView for MenuBar<Key> {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
        self.layout();
    }

    /// Doesn't include any drop down menus
    /// See `full_bounds` for current total size
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        let hovering = self.bounds.contains(mouse.xy);
        let (error, disabled) = self.state.get_err_dis();
        if let Some(bg) = self.style.background.get(hovering, error, disabled) {
            graphics.draw_rect(self.bounds.clone(), fill(bg));
        }
        draw_titles(graphics, mouse.xy, &self.style, &self.items);
    }

    fn update(&mut self, _: &Timing) {}

    fn set_state(&mut self, new_state: ViewState) {
        self.state = new_state;
    }

    fn get_state(&self) -> ViewState {
        self.state
    }
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> LayoutView for MenuBar<Key> {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.layout();
    }
}
