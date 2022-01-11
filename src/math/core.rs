use crate::math::{Point, Rect, UPoint, URect};


impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub const fn zero() -> Self { Point::new(0, 0) }
}

impl UPoint {
    pub const fn new(x: usize, y: usize) -> Self {
        UPoint { x, y }
    }

    pub const fn zero() -> Self { UPoint::new(0, 0) }
}

impl URect {
    pub const fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        URect { x1, y1, x2, y2 }
    }

    pub const fn from_point(point: UPoint, w: usize, h: usize) -> Self {
        URect {
            x1: point.x,
            y1: point.y,
            x2: point.x + w,
            y2: point.y + h,
        }
    }
}

impl Rect {
    pub const fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Self {
        Rect { x1, y1, x2, y2 }
    }

    pub const fn from_point(point: Point, w: isize, h: isize) -> Self {
        Rect {
            x1: point.x,
            y1: point.y,
            x2: point.x + w,
            y2: point.y + h,
        }
    }
}

impl Point {
    pub const fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }
}

impl Rect {
    pub const fn width(&self) -> isize {
        self.x2 - self.x1
    }

    pub const fn height(&self) -> isize {
        self.y2 - self.y1
    }

    pub fn clip_to_urect(&self) -> URect {
        URect::new(
            self.x1.max(0) as usize,
            self.y1.max(0) as usize,
            self.x2.max(0) as usize,
            self.y2.max(0) as usize,
        )
    }

    pub fn move_to(&self, x: isize, y: isize) -> Rect {
        let w = self.width();
        let h = self.height();
        Rect::new(x, y, x + w, y + h)
    }

    pub fn translate(&self, x: isize, y: isize) -> Rect {
        Rect::new(self.x1 + x, self.y1 + y, self.x2 + x, self.y2 + y)
    }

    pub fn topleft(&self) -> Point {
        Point::new(self.x1, self.y1)
    }

    pub fn bottomright(&self) -> Point {
        Point::new(self.x2, self.y2)
    }

    /// Union this rect and another, the result will contain both rectangles
    /// Generally, this means the result will be bigger than self
    pub fn union(&self, other: &Rect) -> Rect {
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
        let x2 = self.x2.max(other.x2);
        let y2 = self.y2.max(other.y2);

        Rect::new(x1, y1, x2, y2)
    }

    /// Intersect this rect and another, the result will contain the area covered by both rectangles
    /// Generally, this means the result will be smaller than self
    ///
    /// # Returns
    ///
    /// self if rectangles do not intersect
    pub fn intersect(&self, other: &Rect) -> Rect {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        if x1 < x2 && y1 < y2 {
            Rect::new(x1, y1, x2, y2)
        } else {
            *self
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);

        x1 < x2 && y1 < y2
    }
}

impl URect {
    pub const fn width(&self) -> usize {
        self.x2 - self.x1
    }

    pub const fn height(&self) -> usize {
        self.y2 - self.y1
    }

    pub fn move_to(&self, x: usize, y: usize) -> URect {
        let w = self.width();
        let h = self.height();
        URect::new(x, y, x + w, y + h)
    }

    pub fn translate(&self, x: isize, y: isize) -> Result<URect, String> {
        if self.x1 as isize + x < 0 {
            return Err(format!(
                "Translated Rect {},{}-{},{} by {},{} which would set x below 0",
                self.x1, self.y1, self.x2, self.y2, x, y
            ));
        }
        if self.y1 as isize + y < 0 {
            return Err(format!(
                "Translated Rect {},{}-{},{} by {},{} which would set y below 0",
                self.x1, self.y1, self.x2, self.y2, x, y
            ));
        }
        Ok(URect::new(
            ((self.x1 as isize) + x) as usize,
            ((self.y1 as isize) + y) as usize,
            ((self.x2 as isize) + x) as usize,
            ((self.y2 as isize) + y) as usize,
        ))
    }

    pub fn topleft(&self) -> UPoint {
        UPoint::new(self.x1, self.y1)
    }

    pub fn bottomright(&self) -> UPoint {
        UPoint::new(self.x2, self.y2)
    }

    /// Union this rect and another, the result will contain both rectangles
    /// Generally, this means the result will be bigger than self
    pub fn union(&self, other: &URect) -> URect {
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
        let x2 = self.x2.max(other.x2);
        let y2 = self.y2.max(other.y2);

        URect::new(x1, y1, x2, y2)
    }

    /// Intersect this rect and another, the result will contain the area covered by both rectangles
    /// Generally, this means the result will be smaller than self
    ///
    /// # Returns
    ///
    /// self if rectangles do not intersect
    pub fn intersect(&self, other: &URect) -> URect {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        if x1 < x2 && y1 < y2 {
            URect::new(x1, y1, x2, y2)
        } else {
            *self
        }
    }

    pub fn intersects(&self, other: &URect) -> bool {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);

        x1 < x2 && y1 < y2
    }
}

#[cfg(test)]
mod test {
    mod rect {
        use crate::math::URect;

        #[test]
        fn width() {
            let rect = URect::new(10, 10, 20, 20);
            assert_eq!(rect.width(), 10);
        }

        #[test]
        fn height() {
            let rect = URect::new(10, 10, 20, 20);
            assert_eq!(rect.height(), 10);
        }

        #[test]
        fn translate() {
            let rect = URect::new(10, 10, 20, 20);
            let translated = rect.translate(5, 5).unwrap();
            assert_eq!(translated, URect::new(15, 15, 25, 25));

            let rect = URect::new(10, 10, 20, 20);
            let translated = rect.translate(-5, -5).unwrap();
            assert_eq!(translated, URect::new(5, 5, 15, 15));

            let rect = URect::new(5, 5, 10, 10);
            let err = rect.translate(-10, -10);
            assert!(err.is_err());
        }

        #[test]
        fn union() {
            let rect = URect::new(10, 10, 20, 20);
            let other = URect::new(15, 15, 25, 25);

            let union = rect.union(&other);

            assert_eq!(union, URect::new(10, 10, 25, 25));

            let rect = URect::new(50, 1, 50, 100);
            let other = URect::new(1, 50, 100, 50);

            let union = rect.union(&other);

            assert_eq!(union, URect::new(1, 1, 100, 100));
        }

        #[test]
        fn intersects() {
            let rect = URect::new(10, 10, 20, 20);
            let does_intersect = URect::new(15, 15, 25, 25);
            let doesnt_intersect = URect::new(30, 30, 40, 40);

            assert!(rect.intersects(&does_intersect));
            assert!(!rect.intersects(&doesnt_intersect));
        }

        #[test]
        fn intersect() {
            let rect = URect::new(10, 10, 20, 20);
            let other = URect::new(15, 15, 25, 25);

            let intersection = rect.intersect(&other);

            assert_eq!(intersection, URect::new(15, 15, 20, 20));

            let rect = URect::new(50, 1, 51, 100);
            let other = URect::new(1, 50, 100, 51);

            let intersection = rect.intersect(&other);

            assert_eq!(intersection, URect::new(50, 50, 51, 51));

            let rect = URect::new(10, 10, 20, 20);
            let doesnt_intersect = URect::new(30, 30, 40, 40);

            assert_eq!(rect.intersect(&doesnt_intersect), rect);
        }
    }
}
