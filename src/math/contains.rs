use crate::math::{Rect, URect};

pub trait Contains<T> {
    fn contains(&self, x: T, y: T) -> bool;
}

impl Contains<usize> for URect {
    #[inline]
    fn contains(&self, x: usize, y: usize) -> bool {
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }
}

impl Contains<isize> for URect {
    #[inline]
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x1 as isize <= x
            && x <= self.x2 as isize
            && self.y1 as isize <= y
            && y <= self.y2 as isize
    }
}

impl Contains<usize> for Rect {
    #[inline]
    fn contains(&self, x: usize, y: usize) -> bool {
        self.x1 <= x as isize
            && x as isize <= self.x2
            && self.y1 <= y as isize
            && y as isize <= self.y2
    }
}

impl Contains<isize> for Rect {
    #[inline]
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_isize_urect() {
        let rect = URect::new(50, 50, 100, 100);
        assert!(!rect.contains(-60_isize, 60));
        assert!(!rect.contains(0_isize, 0));
        assert!(rect.contains(60_isize, 60));
        assert!(!rect.contains(150_isize, 60));
    }

    #[test]
    fn contains_usize_urect() {
        let rect = URect::new(50, 50, 100, 100);
        assert!(!rect.contains(0_usize, 0));
        assert!(rect.contains(60_usize, 60));
        assert!(!rect.contains(150_usize, 60));
    }

    #[test]
    fn contains_isize_rect() {
        let rect = Rect::new(50, 50, 100, 100);
        assert!(!rect.contains(-60_isize, 60));
        assert!(!rect.contains(0_isize, 0));
        assert!(rect.contains(60_isize, 60));
        assert!(!rect.contains(150_isize, 60));
    }

    #[test]
    fn contains_usize_rect() {
        let rect = Rect::new(50, 50, 100, 100);
        assert!(!rect.contains(0_usize, 0));
        assert!(rect.contains(60_usize, 60));
        assert!(!rect.contains(150_usize, 60));
    }
}
