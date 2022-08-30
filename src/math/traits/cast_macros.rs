use crate::math::{Point, UPoint};

macro_rules! impl_cast_tuple {
    ($source:ty, $value:ty, $type:ty) => {
        impl From<($value, $value)> for $source {
            fn from(value: ($value, $value)) -> Self {
                <$source>::new(value.0 as $type, value.1 as $type)
            }
        }
    };
}

macro_rules! impl_cast_rev_tuple {
    ($source:ty, $value:ty) => {
        impl From<$source> for ($value, $value) {
            fn from(value: $source) -> Self {
                (value.x as $value, value.y as $value)
            }
        }
    };
}

macro_rules! impl_tuples {
    ($source:ty, $field:ty, $($t:ty)+) => ($(
        impl_cast_tuple!($source, $t, $field);
        impl_cast_rev_tuple!($source, $t);
    )*)
}

impl_tuples!(Point, isize, u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
impl_tuples!(UPoint, usize, u8 u16 u32 u64 usize);
