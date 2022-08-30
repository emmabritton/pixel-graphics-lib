pub mod contains;
pub mod core;
pub mod lerp;
pub mod traits;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct UPoint {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct URect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}
