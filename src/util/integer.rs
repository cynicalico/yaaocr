use std::ops::*;

pub trait Integer<T>:
    Copy
    + From<u8>
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + Div<Output = T>
    + Mul<Output = T>
    + Rem<Output = T>
    + Sub<Output = T>
    + BitAnd<Output = T>
    + BitOr<Output = T>
    + BitXor<Output = T>
    + Not<Output = T>
    + Shl<u32, Output = T>
    + Shr<u32, Output = T>
{
    const ZERO: T;
    const ONE: T;
    const TEN: T;

    fn trailing_zeros(self) -> u32;
}

macro_rules! impl_integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;

            #[inline]
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }
        }
    )*)
}

impl_integer!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128);

pub trait Unsigned<T>: Integer<T> {}

pub trait Signed<T>: Integer<T> + Neg<Output = T> {}

macro_rules! impl_signedness {
    ($name:ident for $($t:ty)*) => ($(
        impl $name<$t> for $t {}
    )*)
}

impl_signedness!(Unsigned for u8 u16 u32 u64 u128 usize);
impl_signedness!(Signed for i16 i32 i64 i128);
