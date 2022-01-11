use crate::math::traits::*;

//OPS

/// Implement a `std::ops` method (add, sub, mul, etc) on a type with x and y fields
/// where the RHS is a struct with x and y fields too and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Add`)
/// $method = The method to be implemented on the fields (e.g. `add`)
///
/// Example: `impl_ops_struct!(Point, usize, usize, Add, add)`
macro_rules! impl_ops_struct {
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

/// Implement a `std::ops` method (add, sub, mul, etc) on a type with x and y fields
/// where the RHS is a tuple with at least two fields and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Add`)
/// $method = The method to be implemented on the fields (e.g. `add`)
///
/// Example: `impl_ops_tuple!(Point, usize, usize, Add, add)`
macro_rules! impl_ops_tuple {
    ($source:ty, $value:ty, $type:ty, $trait:ident, $method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $method(&self, value: $value) -> Self {
                Self {
                    x: std::ops::$trait::$method(self.x, value.0 as $type),
                    y: std::ops::$trait::$method(self.y, value.1 as $type),
                }
            }
        }
    };
}

/// Implement a `std::ops` method (add, sub, mul, etc) on a type with x and y fields
/// where the RHS is a number and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Add`)
/// $method = The method to be implemented on the fields (e.g. `add`)
///
/// Example: `impl_ops_primitive!(Point, usize, usize, Add, add)`
macro_rules! impl_ops_primitive {
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




// COMPARE

/// Implement a `std::cmp` method (min, max, etc) on a type with x and y fields
/// where the RHS is a struct with x and y fields too and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Max`)
/// $method = The method to be implemented on the fields (e.g. `max`)
///
/// Example: `impl_cmp_struct!(Point, usize, usize, max, MaxXy)`
macro_rules! impl_cmp_struct {
    ($source:ty, $value:ty, $type:ty, $method:ident, $trait:ident, $trait_method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $trait_method(&self, value: $value) -> Self {
                Self {
                    x: std::cmp::$method(self.x, value.x as $type),
                    y: std::cmp::$method(self.y, value.y as $type),
                }
            }
        }
    };
}

/// Implement a `std::cmp` method (min, max, etc) on a type with x and y fields
/// where the RHS is a tuple with at least two fields and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Max`)
/// $method = The method to be implemented on the fields (e.g. `max`)
///
/// Example: `impl_cmp_tuple!(Point, usize, usize, max, MaxXy)`
macro_rules! impl_cmp_tuple {
    ($source:ty, $value:ty, $type:ty, $method:ident, $trait:ident, $trait_method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $trait_method(&self, value: $value) -> Self {
                Self {
                    x: std::cmp::$method(self.x, value.0 as $type),
                    y: std::cmp::$method(self.y, value.1 as $type),
                }
            }
        }
    };
}

/// Implement a `std::cmp` method (min, max, etc) on a type with x and y fields
/// where the RHS is a number and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
/// $trait = The trait to be implemented on the fields (e.g. `Max`)
/// $method = The method to be implemented on the fields (e.g. `max`)
///
/// Example: `impl_cmp_primitive!(Point, usize, usize, max, MaxXy)`
macro_rules! impl_cmp_primitive {
    ($source:ty, $value:ty, $type:ty, $method:ident, $trait:ident, $trait_method:ident) => {
        impl $trait<$value> for $source {
            #[inline]
            fn $trait_method(&self, value: $value) -> Self {
                Self {
                    x: std::cmp::$method(self.x, value as $type),
                    y: std::cmp::$method(self.y, value as $type),
                }
            }
        }
    };
}



// CLAMP

/// Implement clamp on a type with x and y fields
/// where the RHS is a struct with x and y fields too and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
///
/// Example: `impl_clamp_struct!(Point, usize, usize)`
macro_rules! impl_clamp_struct {
    ($source:ty, $value:ty, $type:ty) => {
        impl ClampXy<$value> for $source {
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

/// Implement clamp on a type with x and y fields
/// where the RHS is a struct with x and y fields too and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
///
/// Example: `impl_clamp_tuple!(Point, usize, usize)`
macro_rules! impl_clamp_tuple {
    ($source:ty, $value:ty, $type:ty) => {
        impl ClampXy<$value> for $source {
            #[inline]
            fn clamp_xy(&self, min: $value, max: $value) -> Self {
                Self {
                    x: self.x.clamp(min.0 as $type, max.0 as $type),
                    y: self.y.clamp(min.1 as $type, max.1 as $type),
                }
            }
        }
    };
}

/// Implement clamp on a type with x and y fields
/// where the RHS is a struct with x and y fields too and the result can't fail
/// $source = Self (e.g. Point)
/// $value = Type of RHS fields (e.g. usize)
/// $type = Type of Self fields (e.g. usize)
///
/// Example: `impl_clamp_primitive!(Point, usize, usize)`
macro_rules! impl_clamp_primitive {
    ($source:ty, $value:ty, $type:ty) => {
        impl ClampXy<$value> for $source {
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





macro_rules! impl_primitives {
    ($type: ty, $field: ty, $($t:ty)*) => ($(
        impl_ops_primitive!($type, $t, $field, Add, add);
        impl_ops_primitive!($type, $t, $field, Sub, sub);
        impl_ops_primitive!($type, $t, $field, Mul, mul);

        impl_ops_tuple!($type, ($t,$t), $field, Add, add);
        impl_ops_tuple!($type, ($t,$t), $field, Sub, sub);
        impl_ops_tuple!($type, ($t,$t), $field, Mul, mul);

        impl_cmp_primitive!($type, $t, $field, min, MinXy, min_xy);
        impl_cmp_primitive!($type, $t, $field, max, MaxXy, max_xy);

        impl_cmp_tuple!($type, ($t,$t), $field, min, MinXy, min_xy);
        impl_cmp_tuple!($type, ($t,$t), $field, max, MaxXy, max_xy);

        impl_clamp_primitive!($type, $t, $field);

        impl_clamp_tuple!($type, ($t, $t), $field);
    )*)
}

macro_rules! impl_structs {
    ($type: ty, $field: ty, $($t:ty)*) => ($(
        impl_ops_struct!($type, $t, $field, Add, add);
        impl_ops_struct!($type, $t, $field, Sub, sub);
        impl_ops_struct!($type, $t, $field, Mul, mul);

        impl_cmp_struct!($type, $t, $field, min, MinXy, min_xy);
        impl_cmp_struct!($type, $t, $field, max, MaxXy, max_xy);

        impl_clamp_struct!($type, $t, $field);
    )*)
}

impl_structs!(Point, isize, Point UPoint);
impl_structs!(UPoint, usize, UPoint);

impl_primitives!(Point, isize, usize u8 u16 u32 u64 isize i8 i16 i32 i64);
impl_primitives!(UPoint, usize, usize u8 u16 u32 u64);
