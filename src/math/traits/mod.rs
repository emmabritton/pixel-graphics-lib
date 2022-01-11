mod math_macros;
mod impls;
mod cast_macros;

use crate::math::{Point, UPoint};

pub trait Add<T> {
    /// Return self.x + value and self.y + value
    /// If value is primitive then both `self.x` and `self.y` will be added using the same value otherwise
    /// `self.x` will only added by `min.x` and `max.x` and the same for `y`
    fn add(&self, value: T) -> Self;
}

pub trait Sub<T> {
    /// Return self.x - value and self.y - value
    /// If value is primitive then both `self.x` and `self.y` will be subtracted using the same value otherwise
    /// `self.x` will only subtracted by `min.x` and `max.x` and the same for `y`
    fn sub(&self, value: T) -> Self;
}

pub trait Mul<T> {
    /// Return self.x * value and self.y * value
    /// If value is primitive then both `self.x` and `self.y` will be multiplied using the same value otherwise
    /// `self.x` will only multiplied by `min.x` and `max.x` and the same for `y`
    fn mul(&self, value: T) -> Self;
}

pub trait MinXy<T> {
    /// Return self.x.min(value) and self.y.min(value)
    /// as opposed to min which returns the lowest out of self or value
    /// If value is primitive then both `self.x` and `self.y` will be min'd using the same value otherwise
    /// `self.x` will only min'd by `min.x` and `max.x` and the same for `y`
    fn min_xy(&self, value: T) -> Self;
}

pub trait MaxXy<T> {
    /// Return self.x.max(value) and self.y.max(value)
    /// as opposed to max which returns the highest out of self or value
    /// If value is primitive then both `self.x` and `self.y` will be max'd using the same value otherwise
    /// `self.x` will only max'd by `min.x` and `max.x` and the same for `y`
    fn max_xy(&self, value: T) -> Self;
}

pub trait ClampXy<T> {
    /// Return self.x and self.y clamped between min and max (both inclusive)
    /// min and max can be primitives (such as [usize]) or structs with x and y fields (such as [Point])
    /// If they are primitive then both `self.x` and `self.y` will be clamped using the same value otherwise
    /// `self.x` will only clamped by `min.x` and `max.x` and the same for `y`
    fn clamp_xy(&self, min: T, max: T) -> Self;
}

pub trait Swap {
    ///Swap x and y
    fn swap(&self) -> Self;
}

#[cfg(test)]
mod test {
    use super::*;

    fn point() -> Point {
        Point::new(1, 2)
    }

    fn upoint() -> UPoint {
        UPoint::new(1, 2)
    }

    #[test]
    fn check_point_method_has_not_changed() {
        assert_eq!(point(), Point::new(1, 2));
    }

    #[test]
    fn swaps() {
        assert_eq!(Point::new(5,10).swap(), Point::new(10, 5));
        assert_eq!(UPoint::new(2,8).swap(), UPoint::new(8, 2));
    }

    #[allow(unused_must_use)]
    mod check_all_methods {
        use crate::math::{Point, UPoint};
        use crate::math::traits::{Add, ClampXy, MaxXy, MinXy, Mul, Sub};
        use crate::math::traits::test::{point, upoint};

        #[test]
        fn casts() {
            Point::from(upoint());
            UPoint::try_from(point()).unwrap();
        }

        #[test]
        fn usize() {
            point().add(1_usize);
            point().sub(1_usize);
            point().mul(1_usize);
            point().min_xy(1_usize);
            point().max_xy(1_usize);
            upoint().add(1_usize);
            upoint().sub(1_usize);
            upoint().mul(1_usize);
            upoint().min_xy(1_usize);
            upoint().max_xy(1_usize);

            point().add((1_usize,1));
            point().sub((1_usize,1));
            point().mul((1_usize,1));
            point().min_xy((1_usize,1));
            point().max_xy((1_usize,1));
            upoint().add((1_usize,1));
            upoint().sub((1_usize,1));
            upoint().mul((1_usize,1));
            upoint().min_xy((1_usize,1));
            upoint().max_xy((1_usize,1));

            upoint().add(upoint());

            Point::from((1_usize, 1));
            UPoint::from((1_usize, 1));
            <(usize, usize)>::from(point());
            <(usize, usize)>::from(upoint());

            point().clamp_xy(0_usize, 5);
            upoint().clamp_xy(0_usize, 5);
        }

        #[test]
        fn u8() {
            point().add(1_u8);
            point().sub(1_u8);
            point().mul(1_u8);
            point().min_xy(1_u8);
            point().max_xy(1_u8);
            upoint().add(1_u8);
            upoint().sub(1_u8);
            upoint().mul(1_u8);
            upoint().min_xy(1_u8);
            upoint().max_xy(1_u8);

            point().add((1_u8, 1));
            point().sub((1_u8, 1));
            point().mul((1_u8, 1));
            point().min_xy((1_u8, 1));
            point().max_xy((1_u8, 1));
            upoint().add((1_u8, 1));
            upoint().sub((1_u8, 1));
            upoint().mul((1_u8, 1));
            upoint().min_xy((1_u8, 1));
            upoint().max_xy((1_u8, 1));

            Point::from((1_u8, 1));
            UPoint::from((1_u8, 1));
            <(u8, u8)>::from(point());
            <(u8, u8)>::from(upoint());

            point().clamp_xy(0_u8, 5);
            upoint().clamp_xy(0_u8, 5);
        }

        #[test]
        fn u16() {
            point().add(1_u16);
            point().sub(1_u16);
            point().mul(1_u16);
            point().min_xy(1_u16);
            point().max_xy(1_u16);
            upoint().add(1_u16);
            upoint().sub(1_u16);
            upoint().mul(1_u16);
            upoint().min_xy(1_u16);
            upoint().max_xy(1_u16);

            point().add((1_u16,1));
            point().sub((1_u16,1));
            point().mul((1_u16,1));
            point().min_xy((1_u16,1));
            point().max_xy((1_u16,1));
            upoint().add((1_u16,1));
            upoint().sub((1_u16,1));
            upoint().mul((1_u16,1));
            upoint().min_xy((1_u16,1));
            upoint().max_xy((1_u16,1));

            Point::from((1_u16, 1));
            UPoint::from((1_u16, 1));
            <(u16, u16)>::from(point());
            <(u16, u16)>::from(upoint());

            point().clamp_xy(0_u16, 5);
            upoint().clamp_xy(0_u16, 5);
        }

        #[test]
        fn u32() {
            point().add(1_u32);
            point().sub(1_u32);
            point().mul(1_u32);
            point().min_xy(1_u32);
            point().max_xy(1_u32);
            upoint().add(1_u32);
            upoint().sub(1_u32);
            upoint().mul(1_u32);
            upoint().min_xy(1_u32);
            upoint().max_xy(1_u32);

            point().add((1_u32,1));
            point().sub((1_u32,1));
            point().mul((1_u32,1));
            point().min_xy((1_u32,1));
            point().max_xy((1_u32,1));
            upoint().add((1_u32,1));
            upoint().sub((1_u32,1));
            upoint().mul((1_u32,1));
            upoint().min_xy((1_u32,1));
            upoint().max_xy((1_u32,1));

            Point::from((1_u32, 1));
            UPoint::from((1_u32, 1));
            <(u32, u32)>::from(point());
            <(u32, u32)>::from(upoint());

            point().clamp_xy(0_u32, 5);
            upoint().clamp_xy(0_u32, 5);
        }

        #[test]
        fn u64() {
            point().add(1_u64);
            point().sub(1_u64);
            point().mul(1_u64);
            point().min_xy(1_u64);
            point().max_xy(1_u64);
            upoint().add(1_u64);
            upoint().sub(1_u64);
            upoint().mul(1_u64);
            upoint().min_xy(1_u64);
            upoint().max_xy(1_u64);

            point().add((1_u64,1));
            point().sub((1_u64,1));
            point().mul((1_u64,1));
            point().min_xy((1_u64,1));
            point().max_xy((1_u64,1));
            upoint().add((1_u64,1));
            upoint().sub((1_u64,1));
            upoint().mul((1_u64,1));
            upoint().min_xy((1_u64,1));
            upoint().max_xy((1_u64,1));

            Point::from((1_u64, 1));
            UPoint::from((1_u64, 1));
            <(u64, u64)>::from(point());
            <(u64, u64)>::from(upoint());

            point().clamp_xy(0_u64, 5);
            upoint().clamp_xy(0_u64, 5);
        }

        #[test]
        fn isize() {
            point().add(1_isize);
            point().sub(1_isize);
            point().mul(1_isize);
            point().min_xy(1_isize);
            point().max_xy(1_isize);

            point().add((1_isize, 1));
            point().sub((1_isize, 1));
            point().mul((1_isize, 1));
            point().min_xy((1_isize, 1));
            point().max_xy((1_isize, 1));

            point().add(point());

            Point::from((1_isize, -1));
            <(isize, isize)>::from(point());

            point().clamp_xy(0_isize, 5);
        }

        #[test]
        fn i8() {
            point().add(1_i8);
            point().sub(1_i8);
            point().mul(1_i8);
            point().min_xy(1_i8);
            point().max_xy(1_i8);

            point().add((1_i8, 1));
            point().sub((1_i8, 1));
            point().mul((1_i8, 1));
            point().min_xy((1_i8, 1));
            point().max_xy((1_i8, 1));

            Point::from((1_i8, -1));
            <(i8, i8)>::from(point());

            point().clamp_xy(0_i8, 5);
        }

        #[test]
        fn i16() {
            point().add(1_i16);
            point().sub(1_i16);
            point().mul(1_i16);
            point().min_xy(1_i16);
            point().max_xy(1_i16);

            point().add((1_i16, 1));
            point().sub((1_i16, 1));
            point().mul((1_i16, 1));
            point().min_xy((1_i16, 1));
            point().max_xy((1_i16, 1));

            Point::from((1_i16, -1));
            <(i16, i16)>::from(point());

            point().clamp_xy(0_i16, 5);
        }

        #[test]
        fn i32() {
            point().add(1_i32);
            point().sub(1_i32);
            point().mul(1_i32);
            point().min_xy(1_i32);
            point().max_xy(1_i32);

            point().add((1_i32, 1));
            point().sub((1_i32, 1));
            point().mul((1_i32, 1));
            point().min_xy((1_i32, 1));
            point().max_xy((1_i32, 1));

            Point::from((1_i32, -1));
            <(i32, i32)>::from(point());

            point().clamp_xy(0_i32, 5);
        }

        #[test]
        fn i64() {
            point().add(1_i64);
            point().sub(1_i64);
            point().mul(1_i64);
            point().min_xy(1_i64);
            point().max_xy(1_i64);

            point().add((1_i64, 1));
            point().sub((1_i64, 1));
            point().mul((1_i64, 1));
            point().min_xy((1_i64, 1));
            point().max_xy((1_i64, 1));

            Point::from((1_i64, -1));
            <(i64, i64)>::from(point());

            point().clamp_xy(0_i64, 5);
        }
    }

    #[test]
    fn check_impl() {
        assert_eq!(point().add(3), Point::new(4,5));
        assert_eq!(point().sub(3), Point::new(-2,-1));
        assert_eq!(point().mul(3), Point::new(3,6));
        assert_eq!(point().min_xy(1), Point::new(1,1));
        assert_eq!(point().max_xy(2), Point::new(2,2));
    }
}