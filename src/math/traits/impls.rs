use crate::math::{Point, UPoint};
use crate::math::traits::Swap;

impl Swap for Point {
    fn swap(&self) -> Self {
        Point::new(self.y, self.x)
    }
}

impl Swap for UPoint {
    fn swap(&self) -> Self {
        UPoint::new(self.y, self.x)
    }
}

impl From<UPoint> for Point {
    fn from(point: UPoint) -> Self {
        Point::new(point.x as isize, point.y as isize)
    }
}

impl TryFrom<Point> for UPoint {
    type Error = String;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        if value.x >= 0 || value.y >= 0 {
            Ok(UPoint::new(value.x as usize, value.y as usize))
        } else {
            Err(String::from("Point x and y must be >= 0"))
        }
    }
}