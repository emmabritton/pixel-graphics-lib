use crate::math::{Point, Vec2};

pub trait Add<T> {
    fn add(&self, value: T) -> Self;
}

pub trait Mul<T> {
    fn mul(&self, value: T) -> Self;
}

pub trait Minimize<T> {
    /// Return self.x.min(value.x) and self.y.min(value.y)
    /// as opposed to min which returns the lowest out of self or value
    fn min_xy(&self, value: T) -> Self;
}

pub trait Maximize<T> {
    /// Return self.x.max(value.x) and self.y.max(value.y)
    /// as opposed to max which returns the highest out of self or value
    fn max_xy(&self, value: T) -> Self;
}

pub trait Clamp<T> {
    fn clamp_xy(&self, min: T, max: T) -> Self;
}

pub trait Sub<T> {
    fn sub(&self, value: T) -> Self;
}

macro_rules! impl_clamp_single {
    ($source:ty, $value:ty, $type:ty) => {
        impl Clamp<$value> for $source {
            #[inline]
            fn clamp_xy(&self, min: $value, max: $value) -> Self {
                Self {
                    x: self.x.clamp(min.x as $type, max.x as $type),
                    y: self.y.clamp(min.y as $type, max.y as $type),
                }
            }
        }
    };
}

macro_rules! impl_clamp_both {
    ($source:ty, $value:ty, $type:ty) => {
        impl Clamp<$value> for $source {
            #[inline]
            fn clamp_xy(&self, min: $value, max: $value) -> Self {
                Self {
                    x: self.x.clamp(min as $type, max as $type),
                    y: self.y.clamp(min as $type, max as $type),
                }
            }
        }
    };
}

macro_rules! impl_single {
    ($source:ty, $value:ty, $type:ty, $trait:ident, $method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $method(&self, value: $value) -> Self {
                Self {
                    x: std::ops::$trait::$method(self.x, value.x as $type),
                    y: std::ops::$trait::$method(self.y, value.y as $type),
                }
            }
        }
    };
}

macro_rules! impl_both {
    ($source:ty, $value:ty, $type:ty, $trait:ident, $method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $method(&self, value: $value) -> Self {
                Self {
                    x: std::ops::$trait::$method(self.x, value as $type),
                    y: std::ops::$trait::$method(self.y, value as $type),
                }
            }
        }
    };
}

macro_rules! impl_cmp_single {
    ($source:ty, $value:ty, $type:ty, $trait:ident, $cmp_method:ident, $method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $method(&self, value: $value) -> Self {
                Self {
                    x: std::cmp::$cmp_method(self.x, value.x as $type),
                    y: std::cmp::$cmp_method(self.y, value.y as $type),
                }
            }
        }
    };
}

macro_rules! impl_cmp_both {
    ($source:ty, $value:ty, $type:ty, $trait:ident, $cmp_method:ident, $method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $method(&self, value: $value) -> Self {
                Self {
                    x: std::cmp::$cmp_method(self.x, value as $type),
                    y: std::cmp::$cmp_method(self.y, value as $type),
                }
            }
        }
    };
}

macro_rules! impl_set {
    ($trait:ident, $method:ident) => {
        impl_single!(Vec2, Vec2, isize, $trait, $method);
        impl_both!(Vec2, isize, isize, $trait, $method);
        impl_both!(Vec2, i32, isize, $trait, $method);
        impl_both!(Vec2, usize, isize, $trait, $method);

        impl_single!(Point, Point, usize, $trait, $method);
        impl_both!(Point, usize, usize, $trait, $method);
    };
}

macro_rules! impl_cmp_set {
    ($trait:ident, $cmp_method:ident, $method:ident) => {
        impl_cmp_single!(Vec2, Vec2, isize, $trait, $cmp_method, $method);
        impl_cmp_both!(Vec2, isize, isize, $trait, $cmp_method, $method);
        impl_cmp_both!(Vec2, i32, isize, $trait, $cmp_method, $method);
        impl_cmp_both!(Vec2, usize, isize, $trait, $cmp_method, $method);

        impl_cmp_single!(Point, Point, usize, $trait, $cmp_method, $method);
        impl_cmp_both!(Point, usize, usize, $trait, $cmp_method, $method);
    };
}

impl_set!(Add, add);
impl_set!(Sub, sub);
impl_set!(Mul, mul);

impl_cmp_set!(Minimize, min, min_xy);
impl_cmp_set!(Maximize, max, max_xy);

impl_clamp_single!(Vec2, Vec2, isize);
impl_clamp_both!(Vec2, isize, isize);
impl_clamp_both!(Vec2, i32, isize);
impl_clamp_both!(Vec2, usize, isize);

impl_clamp_single!(Point, Point, usize);
impl_clamp_both!(Point, usize, usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec2() {
        assert_eq!(Vec2::new(1, 1).add(Vec2::new(1, 1)), Vec2::new(2, 2));
        assert_eq!(Vec2::new(1, 1).sub(Vec2::new(1, 1)), Vec2::new(0, 0));
        assert_eq!(Vec2::new(1, 2).mul(Vec2::new(2, 1)), Vec2::new(2, 2));
        assert_eq!(Vec2::new(1, 2).min_xy(Vec2::new(2, 1)), Vec2::new(1, 1));
        assert_eq!(Vec2::new(1, 2).max_xy(Vec2::new(2, 1)), Vec2::new(2, 2));
        assert_eq!(
            Vec2::new(1, 3).clamp_xy(Vec2::new(2, 1), Vec2::new(2, 4)),
            Vec2::new(2, 3)
        );
    }
}
