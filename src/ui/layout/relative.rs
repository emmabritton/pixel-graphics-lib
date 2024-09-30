#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::{coord, Rect, Shape};
use crate::ui::layout::LayoutView;
use crate::ui::PixelView;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LayoutOffset {
    Zero,
    Pixels(isize),
    /// percentage of parent
    Percent(f32),
}

impl LayoutOffset {
    pub fn calc(&self, total: usize) -> isize {
        match self {
            LayoutOffset::Zero => 0,
            LayoutOffset::Pixels(px) => *px,
            LayoutOffset::Percent(percent) => ((total as f32) * percent).round() as isize,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutContext {
    pub bounds: Rect,
    pub default_offset: LayoutOffset,
}

impl LayoutContext {
    pub fn new(bounds: Rect) -> LayoutContext {
        LayoutContext {
            bounds,
            default_offset: LayoutOffset::Zero,
        }
    }

    pub fn new_with_padding(bounds: Rect, padding: usize) -> LayoutContext {
        LayoutContext {
            bounds,
            default_offset: LayoutOffset::Pixels(padding as isize),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ViewLayoutRule {
    LeftToLeft,
    RightToRight,
    LeftToRight,
    RightToLeft,
    TopToTop,
    BottomToBottom,
    TopToBottom,
    BottomToTop,
    CenterHToLeft,
    CenterHToRight,
    CenterHToCenterH,
    LeftToCenterH,
    RightToCenterH,
    CenterVToTop,
    CenterVToBottom,
    TopToCenterV,
    BottomToCenterV,
    CenterVToCenterV,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParentLayoutRule {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
    FromCenterH,
    FromCenterV,
    FillVert,
    FillHorz,
}

trait UpdateRect {
    fn u_left(&self, value: isize) -> Rect;
    fn u_top(&self, value: isize) -> Rect;
    fn u_bottom(&self, value: isize) -> Rect;
    fn u_right(&self, value: isize) -> Rect;
    fn u_horz(&self, left: isize, right: isize) -> Rect;
    fn u_vert(&self, top: isize, bottom: isize) -> Rect;
}

impl UpdateRect for Rect {
    fn u_left(&self, value: isize) -> Rect {
        Rect::new((value, self.top()), self.bottom_right())
    }

    fn u_top(&self, value: isize) -> Rect {
        Rect::new((self.left(), value), self.bottom_right())
    }

    fn u_bottom(&self, value: isize) -> Rect {
        Rect::new(self.top_left(), (self.right(), value))
    }

    fn u_right(&self, value: isize) -> Rect {
        Rect::new(self.top_left(), (value, self.bottom()))
    }

    fn u_horz(&self, left: isize, right: isize) -> Rect {
        Rect::new((left, self.top()), (right, self.bottom()))
    }

    fn u_vert(&self, top: isize, bottom: isize) -> Rect {
        Rect::new((self.left(), top), (self.right(), bottom))
    }
}

pub fn move_by_view(
    parent: &Rect,
    view: &mut dyn PixelView,
    pivot: &dyn PixelView,
    rule: ViewLayoutRule,
    offset: LayoutOffset,
) {
    match rule {
        ViewLayoutRule::LeftToLeft => view.set_position(coord!(
            pivot.bounds().left() + offset.calc(parent.width()),
            view.bounds().top(),
        )),
        ViewLayoutRule::RightToRight => view.set_position(coord!(
            pivot.bounds().right() + offset.calc(parent.width()) - (view.bounds().width() as isize),
            view.bounds().top(),
        )),
        ViewLayoutRule::LeftToRight => view.set_position(coord!(
            pivot.bounds().right() + offset.calc(parent.width()),
            view.bounds().top(),
        )),
        ViewLayoutRule::RightToLeft => view.set_position(coord!(
            pivot.bounds().left() - offset.calc(parent.width()) - (view.bounds().width() as isize),
            view.bounds().top()
        )),
        ViewLayoutRule::TopToTop => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().top() + offset.calc(parent.height())
        )),
        ViewLayoutRule::BottomToBottom => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().bottom()
                - offset.calc(parent.height())
                - (view.bounds().height() as isize)
        )),
        ViewLayoutRule::TopToBottom => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().bottom() + offset.calc(parent.height())
        )),
        ViewLayoutRule::BottomToTop => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().top() - offset.calc(parent.height()) - (view.bounds().height() as isize)
        )),
        ViewLayoutRule::CenterHToLeft => view.set_position(coord!(
            pivot.bounds().left() + offset.calc(parent.width())
                - (view.bounds().width() as isize / 2),
            view.bounds().top()
        )),
        ViewLayoutRule::CenterHToRight => view.set_position(coord!(
            pivot.bounds().left()
                + offset.calc(parent.width())
                + (view.bounds().width() as isize / 2),
            view.bounds().top()
        )),
        ViewLayoutRule::LeftToCenterH => view.set_position(coord!(
            pivot.bounds().center().x + offset.calc(parent.width()),
            view.bounds().top()
        )),
        ViewLayoutRule::RightToCenterH => view.set_position(coord!(
            pivot.bounds().center().x
                - offset.calc(parent.width())
                - (view.bounds().width() as isize),
            view.bounds().top()
        )),
        ViewLayoutRule::CenterVToTop => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().top() + offset.calc(parent.height())
                - (view.bounds().height() as isize / 2)
        )),
        ViewLayoutRule::CenterVToBottom => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().top()
                + offset.calc(parent.height())
                + (view.bounds().height() as isize / 2)
        )),
        ViewLayoutRule::TopToCenterV => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().center().y + offset.calc(parent.height()),
        )),
        ViewLayoutRule::BottomToCenterV => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().center().y + offset.calc(parent.height())
                - (view.bounds().height() as isize),
        )),
        ViewLayoutRule::CenterHToCenterH => view.set_position(coord!(
            pivot.bounds().center().x + offset.calc(parent.width())
                - (view.bounds().width() as isize / 2),
            view.bounds().top()
        )),
        ViewLayoutRule::CenterVToCenterV => view.set_position(coord!(
            view.bounds().left(),
            pivot.bounds().center().y
                - offset.calc(parent.height())
                - (view.bounds().height() as isize / 2),
        )),
    }
}

pub fn center_between(
    parent: &Rect,
    view: &mut dyn LayoutView,
    first: &dyn PixelView,
    second: &dyn PixelView,
    horz: bool,
    offset: LayoutOffset,
) {
    if horz {
        let half_width = (view.bounds().width() / 2) as isize;
        view.set_position(coord!(
            first
                .bounds()
                .center()
                .mid_point(second.bounds().center())
                .x
                - half_width
                + offset.calc(parent.width()),
            view.bounds().top()
        ));
    } else {
        let half_height = (view.bounds().height() / 2) as isize;
        view.set_position(coord!(
            view.bounds().left(),
            first
                .bounds()
                .center()
                .mid_point(second.bounds().center())
                .y
                - half_height
                + offset.calc(parent.height())
        ));
    }
}

pub fn grow_by_view(
    parent: &Rect,
    view: &mut dyn LayoutView,
    pivot: &dyn PixelView,
    rule: ViewLayoutRule,
    offset: LayoutOffset,
) {
    match rule {
        ViewLayoutRule::LeftToLeft => view.set_bounds(
            view.bounds()
                .u_left(pivot.bounds().left() + offset.calc(parent.width())),
        ),
        ViewLayoutRule::RightToRight => view.set_bounds(
            view.bounds()
                .u_right(pivot.bounds().right() - offset.calc(parent.width())),
        ),
        ViewLayoutRule::LeftToRight => view.set_bounds(
            view.bounds()
                .u_left(pivot.bounds().right() + offset.calc(parent.width())),
        ),
        ViewLayoutRule::RightToLeft => view.set_bounds(
            view.bounds()
                .u_right(pivot.bounds().left() - offset.calc(parent.width())),
        ),
        ViewLayoutRule::TopToTop => view.set_bounds(
            view.bounds()
                .u_top(pivot.bounds().top() + offset.calc(parent.height())),
        ),
        ViewLayoutRule::BottomToBottom => view.set_bounds(
            view.bounds()
                .u_bottom(pivot.bounds().bottom() - offset.calc(parent.height())),
        ),
        ViewLayoutRule::TopToBottom => view.set_bounds(
            view.bounds()
                .u_top(pivot.bounds().bottom() + offset.calc(parent.height())),
        ),
        ViewLayoutRule::BottomToTop => view.set_bounds(
            view.bounds()
                .u_bottom(pivot.bounds().top() - offset.calc(parent.height())),
        ),
        ViewLayoutRule::LeftToCenterH => view.set_bounds(
            view.bounds()
                .u_left(pivot.bounds().center().x + offset.calc(parent.width())),
        ),
        ViewLayoutRule::RightToCenterH => view.set_bounds(
            view.bounds()
                .u_right(pivot.bounds().center().x + offset.calc(parent.width())),
        ),
        ViewLayoutRule::TopToCenterV => view.set_bounds(
            view.bounds()
                .u_top(pivot.bounds().center().y + offset.calc(parent.width())),
        ),
        ViewLayoutRule::BottomToCenterV => view.set_bounds(
            view.bounds()
                .u_bottom(pivot.bounds().center().y + offset.calc(parent.width())),
        ),
        _ => {}
    }
}

pub fn move_by_parent(
    parent: &Rect,
    view: &mut dyn PixelView,
    rule: ParentLayoutRule,
    offset: LayoutOffset,
) {
    match rule {
        ParentLayoutRule::FromLeft => view.set_position(coord!(
            parent.left() + offset.calc(parent.width()),
            view.bounds().top(),
        )),
        ParentLayoutRule::FromRight => view.set_position(coord!(
            parent.right() - (view.bounds().width() as isize) - offset.calc(parent.width()),
            view.bounds().top(),
        )),
        ParentLayoutRule::FromTop => view.set_position(coord!(
            view.bounds().left(),
            parent.top() + offset.calc(parent.height()),
        )),
        ParentLayoutRule::FromBottom => view.set_position(coord!(
            view.bounds().left(),
            parent.bottom() - (view.bounds().height() as isize) - offset.calc(parent.height()),
        )),
        ParentLayoutRule::FromCenterH => view.set_position(coord!(
            parent.center().x + offset.calc(parent.width()) - (view.bounds().width() as isize / 2),
            view.bounds().top(),
        )),
        ParentLayoutRule::FromCenterV => view.set_position(coord!(
            view.bounds().left(),
            parent.center().y + offset.calc(parent.height())
                - (view.bounds().height() as isize / 2),
        )),
        _ => {}
    }
}

pub fn grow_by_parent(
    parent: &Rect,
    view: &mut dyn LayoutView,
    rule: ParentLayoutRule,
    offset: LayoutOffset,
) {
    match rule {
        ParentLayoutRule::FromLeft => view.set_bounds(
            view.bounds()
                .u_left(parent.left() + offset.calc(parent.width())),
        ),
        ParentLayoutRule::FromRight => view.set_bounds(
            view.bounds()
                .u_right(parent.right() - offset.calc(parent.width())),
        ),
        ParentLayoutRule::FromTop => view.set_bounds(
            view.bounds()
                .u_top(parent.top() + offset.calc(parent.height())),
        ),
        ParentLayoutRule::FromBottom => view.set_bounds(
            view.bounds()
                .u_bottom(parent.bottom() - offset.calc(parent.height())),
        ),
        ParentLayoutRule::FillVert => view.set_bounds(view.bounds().u_vert(
            parent.top() + offset.calc(parent.height()),
            parent.bottom() - offset.calc(parent.height()),
        )),
        ParentLayoutRule::FillHorz => view.set_bounds(view.bounds().u_horz(
            parent.left() + offset.calc(parent.width()),
            parent.right() - offset.calc(parent.width()),
        )),
        ParentLayoutRule::FromCenterH => {
            if view.bounds().center().x < parent.center().x {
                view.set_bounds(
                    view.bounds()
                        .u_right(parent.center().x + offset.calc(parent.width())),
                )
            } else {
                view.set_bounds(
                    view.bounds()
                        .u_left(parent.center().x - offset.calc(parent.width())),
                )
            }
        }
        ParentLayoutRule::FromCenterV => {
            if view.bounds().center().y < parent.center().y {
                view.set_bounds(
                    view.bounds()
                        .u_bottom(parent.center().y + offset.calc(parent.height())),
                )
            } else {
                view.set_bounds(
                    view.bounds()
                        .u_top(parent.center().y - offset.calc(parent.height())),
                )
            }
        }
    }
}

/// Position and size a view relative to the parent or another view
///
/// # Format
///
/// layout!(context, [command] view, alignment [pivot_view],[second_pivot_view][, offset]);
///
/// Views must impl [PixelView] and to use `grow` they must also impl [LayoutView]
///
/// `offset` replaces the default offset from context (if it was set)
///
/// # Usage
///
/// ```ignore
/// let mut view1 = Button::new(...);
/// let mut view2 = Button::new(...);
/// let context = LayoutContext::new(Rect::new((0,0), (200,200)));
/// layout!(context, view1, left_to_left_of view2);
/// ```
///
/// # Examples
/// Move a button below another with some spacing
/// ```ignore
/// let button1 = Button::new(...);
/// let button2 = Button::new(...);
/// let context = LayoutContext::new(...);
/// layout!(context, button2, top_to_bottom_of button1, px(8));
/// ```
///
/// Move a button to the edge of the screen and grow it's right side to match another view
/// ```ignore
/// let button1 = Button::new(...);
/// let button2 = Button::new(...);
/// let context = LayoutContext::new(...);
/// layout!(context, button2, align_left);
/// layout!(context, grow button1, right_to_right_of button1);
/// ```
///
/// # Command
/// * `grow` - Moves the edge of the view, but not the position
///
/// # Alignment
/// *View*
/// * `left_to_left_of` - Makes view.x = pivot_view.x
/// * `top_to_top_of` - Makes view.y = pivot_view.y
/// * `right_to_right_of` - Makes view.x = pivot_view.right - view.width
/// * `bottom_to_bottom_of` - Makes view.y = pivot_view.bottom - view.height
/// * `left_to_right_of` - Makes view.x = pivot_view.x + pivot_view.width
/// * `right_to_left_of` - Makes view.x = pivot_view.x - view.width
/// * `top_to_bottom_of` - Makes view.y = pivot_view.bottom
/// * `bottom_to_top_of` - Makes view.y = pivot_view.y - view.height
/// * `centerh_to_centerh_of` - Makes view.center.x = pivot_view.center.x (`grow` not supported)
/// * `centerv_to_centerv_of` - Makes view.center.y = pivot_view.center.y (`grow` not supported)
/// * `centerv_to_top_of` - Makes view.center.y = pivot_view.y (`grow` not supported)
/// * `centerv_to_bottom_of` - Makes view.center.y = pivot_view.bottom (`grow` not supported)
/// * `centerh_to_left_of` - Makes view.center.x = pivot_view.left (`grow` not supported)
/// * `centerh_to_right_of` - Makes view.center.x = pivot_view.right (`grow` not supported)
/// * `left_to_centerh_of` - Makes view.x = pivot_view.center.x
/// * `right_to_centerh_of` - Makes view.x = pivot_view.center.x - view.width
/// * `top_to_centerv_of` - Makes view.y = pivot_view.center.y
/// * `bottom_to_centerv_of` - Makes view.y = pivot_view.center.y - view.height
/// * `center_between_horz` - Make view.center.x = pivot_view.right midpoint second_pivot_view.left
/// * `center_between_vert` - Make view.center.y = pivot_view.bottom midpoint second_pivot_view.top
///
/// *Parent*
/// * `fill_width` - Set x to context.left, width to context.width (`grow` only)
/// * `fill_height` - Set y to context.top, height to context.height (`grow` only)
/// * `align_left` - Set x to context.left
/// * `align_right` - Set x to (context.right - view.width)
/// * `align_top` - Set y to context.top
/// * `align_bottom` - Set y to (context.bottom - view.height)
/// * `align_centerh` - Set x to context.center.x (when using `grow` moves view.left or view.right instead)
/// * `align_centerv` - Set y to context.center.y (when using `grow` moves view.top or view.bottom instead)
#[macro_export]
macro_rules! layout {
    ($context:expr, $view:expr, left_to_left_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, left_to_right_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, right_to_left_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, top_to_bottom_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, bottom_to_top_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, top_to_top_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, bottom_to_bottom_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, right_to_right_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, right_to_right_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, right_to_left_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, bottom_to_bottom_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, bottom_to_top_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, left_to_left_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, left_to_right_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, top_to_top_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerh_to_centerh_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterHToCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerh_to_left_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterHToLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerh_to_right_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterHToRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerv_to_centerv_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterVToCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerv_to_top_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterVToTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, centerv_to_bottom_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::CenterVToBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, left_to_centerh_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, right_to_centerh_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, left_to_centerh_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::LeftToCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, right_to_centerh_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::RightToCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, top_to_centerv_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, bottom_to_centerv_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, top_to_centerv_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, bottom_to_centerv_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::BottomToCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, top_to_bottom_of $pivot:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_view(&$context.bounds, &mut $view, &$pivot, $crate::ui::layout::relative::ViewLayoutRule::TopToBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, fill_width $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FillHorz, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, fill_height $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FillVert, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_left $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_top $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_right $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_bottom $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_left $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromLeft, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_top $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromTop, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_right $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromRight, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_bottom $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromBottom, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_centerh $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, $view:expr, align_centerv $(, $offset:expr)?) => {
        $crate::ui::layout::relative::move_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_centerh $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromCenterH, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, grow $view:expr, align_centerv $(, $offset:expr)?) => {
        $crate::ui::layout::relative::grow_by_parent(&$context.bounds, &mut $view, $crate::ui::layout::relative::ParentLayoutRule::FromCenterV, $crate::or_else!($($offset)?, $context.default_offset));
    };
    ($context:expr, centerh $view:expr, between $first:expr, $second:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::center_between(&$context.bounds, &mut $view, &$first, &$second, true, $context.default_offset);
    };
    ($context:expr, centerv $view:expr, between $first:expr, $second:expr $(, $offset:expr)?) => {
        $crate::ui::layout::relative::center_between(&$context.bounds, &mut $view, &$first, &$second, false, $context.default_offset);
    };
}

#[macro_export]
macro_rules! px {
    ($number:expr) => {
        $crate::ui::layout::relative::LayoutOffset::Pixels($number)
    };
}

#[macro_export]
macro_rules! parent {
    ($number:expr) => {
        $crate::ui::layout::relative::LayoutOffset::Percent($number)
    };
}
