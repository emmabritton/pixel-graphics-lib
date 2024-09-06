use crate::ui::prelude::*;
use crate::ui::ViewState;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct MenuItemView<Key: Hash + Copy + PartialEq + Eq + Debug> {
    pub(super) id: Key,
    /// Bounds of this item (checkmark, text and arrow)
    pub(super) item_bounds: Rect,
    /// Label
    pub(super) name: String,
    pub(super) state: ViewState,
    /// True if this or any children are focused
    pub(super) focused: bool,
    pub(super) content: ItemContent<Key>,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum ItemContent<Key: Hash + Copy + PartialEq + Eq + Debug> {
    Button,
    Checkable(bool),
    Parent(Vec<MenuItemView<Key>>, ChildrenAnchor, Rect),
}

/// Should the children be drawn below (i.e. this is a root item)
/// or from the side
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(super) enum ChildrenAnchor {
    Bottom,
    Side,
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> MenuItemView<Key> {
    pub(super) fn new(item: &MenuBarItem<Key>, is_root: bool) -> Self {
        let anchor = if is_root {
            ChildrenAnchor::Bottom
        } else {
            ChildrenAnchor::Side
        };
        let content = if let Some(default) = item.checkable {
            ItemContent::Checkable(default)
        } else if let Some(items) = &item.children {
            let children: Vec<MenuItemView<Key>> =
                items.iter().map(|c| MenuItemView::new(c, false)).collect();
            ItemContent::Parent(children, anchor, Rect::new((0, 0), (0, 0)))
        } else {
            ItemContent::Button
        };

        Self {
            id: item.id,
            item_bounds: Rect::new((0, 0), (0, 0)),
            name: item.name.clone(),
            state: ViewState::Normal,
            focused: false,
            content,
        }
    }
}

impl<Key: Hash + Copy + PartialEq + Eq + Debug> MenuItemView<Key> {
    /// sets focused/expanded as true if this or any child is hovered over
    pub(super) fn on_mouse_move(&mut self, xy: Coord) {
        self.focused = self.item_bounds.contains(xy) || (self.focused && self.child_expanded(xy))
    }

    /// returns true if any child is expanded
    fn child_expanded(&mut self, xy: Coord) -> bool {
        match &mut self.content {
            ItemContent::Checkable(_) | ItemContent::Button => false,
            ItemContent::Parent(items, _, _) => {
                for child in items.iter_mut() {
                    child.on_mouse_move(xy);
                }
                items.iter().any(|c| c.focused)
            }
        }
    }
}

pub(super) fn draw_titles<Key: Hash + Copy + PartialEq + Eq + Debug>(
    graphics: &mut Graphics,
    mouse: Coord,
    style: &MenuBarStyle,
    items: &[MenuItemView<Key>],
) {
    for title in items {
        let col = style.menu_item.font.px_to_cols(title.item_bounds.width());
        let (err, dis) = title.state.get_err_dis();
        let hover = title.item_bounds.contains(mouse);
        if let Some(clr) = style
            .menu_item
            .background
            .get(hover, title.focused, err, dis)
        {
            graphics.draw_rect(title.item_bounds.clone(), fill(clr));
        }
        if let Some(clr) = style.menu_item.text.get(hover, title.focused, err, dis) {
            let str = match title.content {
                ItemContent::Checkable(true) => "✓",
                ItemContent::Checkable(false) => " ",
                _ => "",
            };
            graphics.draw_text(
                &format!("{str}{}", title.name),
                TextPos::px(title.item_bounds.top_left() + style.menu_item.padding.offset()),
                (clr, style.menu_item.font, WrappingStrategy::AtCol(col)),
            );
        }
        draw(graphics, mouse, style, title);
    }
}

fn draw<Key: Hash + Copy + PartialEq + Eq + Debug>(
    graphics: &mut Graphics,
    mouse: Coord,
    style: &MenuBarStyle,
    item: &MenuItemView<Key>,
) {
    if item.focused && item.state == ViewState::Normal {
        if let Some(bg) = style.menu_item.dropdown_background {
            match &item.content {
                ItemContent::Button | ItemContent::Checkable(_) => {}
                ItemContent::Parent(_, _, bounds) => graphics.draw_rect(bounds.clone(), fill(bg)),
            }
        }
        match &item.content {
            ItemContent::Checkable(_) | ItemContent::Button => {}
            ItemContent::Parent(items, _, dropdown_bounds) => {
                let col = style.dropdown_item.font.px_to_cols(dropdown_bounds.width());
                let any_checkable = items
                    .iter()
                    .any(|c| matches!(c.content, ItemContent::Checkable(_)));
                for child in items {
                    let (err, dis) = child.state.get_err_dis();
                    if let Some(clr) = style.dropdown_item.background.get(
                        child.item_bounds.contains(mouse),
                        child.focused,
                        err,
                        dis,
                    ) {
                        graphics.draw_rect(child.item_bounds.clone(), fill(clr));
                    }
                    if let Some(clr) = style.dropdown_item.text.get(
                        child.item_bounds.contains(mouse),
                        child.focused,
                        err,
                        dis,
                    ) {
                        let str = match (any_checkable, &child.content) {
                            (true, ItemContent::Checkable(true)) => "✓",
                            (true, _) => " ",
                            (_, _) => "",
                        };
                        graphics.draw_text(
                            &format!("{str}{}", &child.name),
                            TextPos::px(
                                child.item_bounds.top_left() + style.dropdown_item.padding.offset(),
                            ),
                            (clr, style.dropdown_item.font, WrappingStrategy::Cutoff(col)),
                        );
                    }
                    if matches!(&child.content, ItemContent::Parent(_, _, _)) {
                        if let Some(icon) = style.dropdown_item.arrow.get(
                            child.item_bounds.contains(mouse),
                            child.focused,
                            err,
                            dis,
                        ) {
                            graphics.draw_indexed_image(
                                child.item_bounds.top_right() - (icon.width() as usize, 0)
                                    + (
                                        0,
                                        (child.item_bounds.height() / 2)
                                            - (icon.height() as usize / 2),
                                    ),
                                icon,
                            );
                        }
                        draw(graphics, mouse, style, child);
                    }
                }
            }
        }
    }
}

pub(super) fn collapse_menu<Key: Hash + Copy + PartialEq + Eq + Debug>(
    children: &mut [MenuItemView<Key>],
) {
    children.iter_mut().for_each(|c| {
        if let ItemContent::Parent(items, _, _) = &mut c.content {
            collapse_menu(items);
        }
        c.focused = false;
    });
}

/// returns the path to the view clicked on (if it exists)
/// will return path to options element
pub(super) fn on_click_path<Key: Hash + Copy + PartialEq + Eq + Debug>(
    items: &[MenuItemView<Key>],
    down_at: Coord,
    up_at: Coord,
) -> Option<Key> {
    for item in items {
        if item.focused && item.state == ViewState::Normal {
            if item.item_bounds.contains(down_at) && item.item_bounds.contains(up_at) {
                return Some(item.id);
            } else if let ItemContent::Parent(children, _, _) = &item.content {
                let result = on_click_path(children, down_at, up_at);
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}

pub(super) fn focused_bounds<Key: Hash + Copy + PartialEq + Eq + Debug>(
    item: &MenuItemView<Key>,
) -> Option<Rect> {
    if item.focused && item.state == ViewState::Normal {
        let mut bounds = item.item_bounds.clone();
        match &item.content {
            ItemContent::Checkable(_) | ItemContent::Button => {}
            ItemContent::Parent(items, _, child_bounds) => {
                bounds = union(&bounds, child_bounds);
                for child in items {
                    if let Some(extra) = focused_bounds(child) {
                        bounds = union(&bounds, &extra);
                    }
                }
            }
        }
        Some(bounds)
    } else {
        None
    }
}

pub(super) fn union(lhs: &Rect, rhs: &Rect) -> Rect {
    Rect::new(
        (lhs.left().min(rhs.left()), lhs.top().min(rhs.top())),
        (lhs.right().max(rhs.right()), lhs.bottom().max(rhs.bottom())),
    )
}

pub(super) fn layout_titles<Key: Hash + Copy + PartialEq + Eq + Debug>(
    top_left: Coord,
    style: &MenuBarStyle,
    screen_size: (usize, usize),
    fill_width: bool,
    items: &mut [MenuItemView<Key>],
) -> Rect {
    let mut bounds = Rect::new_with_size(top_left, if fill_width { screen_size.0 } else { 0 }, 0);
    let mut start = top_left;
    for item in items {
        let (w, h) = style.menu_item.font.measure(&item.name);
        item.item_bounds = Rect::new_with_size(
            start,
            w + style.menu_item.padding.horz(),
            h + style.menu_item.padding.vert(),
        );
        start = item.item_bounds.top_right() + (1, 0);
        bounds = union(&bounds, &item.item_bounds);
        match &mut item.content {
            ItemContent::Checkable(_) | ItemContent::Button => {}
            ItemContent::Parent(_, _, _) => layout_children(item, style, screen_size),
        }
    }

    bounds
}

#[allow(clippy::only_used_in_recursion)] //screen_size for future use
fn layout_children<Key: Hash + Copy + PartialEq + Eq + Debug>(
    item: &mut MenuItemView<Key>,
    style: &MenuBarStyle,
    screen_size: (usize, usize),
) {
    match &mut item.content {
        ItemContent::Checkable(_) | ItemContent::Button => {}
        ItemContent::Parent(items, anchor, bounds) => {
            let min_right = if anchor == &ChildrenAnchor::Bottom {
                item.item_bounds.right().max(0) as usize
            } else {
                0
            };
            let any_checks = items
                .iter()
                .any(|v| matches!(v.content, ItemContent::Checkable(_)));
            let any_submenus = items
                .iter()
                .any(|v| matches!(v.content, ItemContent::Parent(_, _, _)));
            let w = items
                .iter()
                .map(|v| {
                    let mut w = style.dropdown_item.font.measure(&v.name).0;
                    if any_checks {
                        w += style.dropdown_item.font.char_width();
                    }
                    if any_submenus {
                        w += style.dropdown_item.font.char_width();
                    }
                    w
                })
                .max()
                .unwrap_or_default();
            let mut start = if anchor == &ChildrenAnchor::Bottom {
                item.item_bounds.bottom_left() + (0, 1)
            } else if item.item_bounds.top_right().x + w as isize > screen_size.0 as isize
                && item.item_bounds.top_left().x - w as isize >= 0
            {
                item.item_bounds.top_left() - (w, 0) - (style.dropdown_item.padding.horz(), 0)
            } else {
                item.item_bounds.top_right()
            };
            let mut container_bounds = Rect::new_with_size(start, w, 0);
            for child in items {
                let (_, h) = style.dropdown_item.font.measure(&child.name);
                child.item_bounds = Rect::new(
                    start,
                    (
                        (start.x + w as isize + style.dropdown_item.padding.horz() as isize)
                            .max(min_right as isize),
                        start.y + h as isize + style.dropdown_item.padding.vert() as isize,
                    ),
                );
                start = child.item_bounds.bottom_left() + (0, 1);
                container_bounds = union(&container_bounds, &child.item_bounds);
                layout_children(child, style, screen_size);
            }
            *bounds = Rect::new(
                container_bounds.top_left(),
                (
                    container_bounds.bottom_right().x.max(min_right as isize),
                    container_bounds.bottom_right().y,
                ),
            );
        }
    }
}
