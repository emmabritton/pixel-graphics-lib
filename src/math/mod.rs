pub mod contains;
pub mod core;
pub mod lerp;
pub mod traits;

use serde::{Deserialize, Serialize};

pub const VEC2_ZERO: Vec2 = Vec2::new(0, 0);
pub const POINT_ZERO: Point = Point::new(0, 0);

/// Vector of 2 isize
/// General used to general pixels
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

/// Vector of 2 usize
/// General used to general logical units
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct URect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
}
