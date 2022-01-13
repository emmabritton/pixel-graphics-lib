use std::ops::Neg;
use crate::math::{Point, UPoint};
use crate::math::traits::{SimpleMath, Swap};

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

impl SimpleMath for Point {
    fn half(&self) -> Self {
        Point::new(self.x / 2, self.y / 2)
    }

    fn double(&self) -> Self {
        Point::new(self.x * 2, self.y * 2)
    }
}

impl SimpleMath for UPoint {
    fn half(&self) -> Self {
        UPoint::new(self.x / 2, self.y / 2)
    }

    fn double(&self) -> Self {
        UPoint::new(self.x * 2, self.y * 2)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(self.x.neg(), self.y.neg())
    }
}