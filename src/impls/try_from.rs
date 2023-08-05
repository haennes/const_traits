use crate::TryFrom;

pub struct TryFromIntError(pub ());

// no possible bounds violation
macro_rules! try_from_unbounded {
    ($source:ty, $($target:ty),*) => {$(
        impl const TryFrom<$source> for $target {
            type Error = TryFromIntError;

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(value: $source) -> Result<Self, Self::Error> {
                Ok(value as Self)
            }
        }
    )*}
}

// only negative bounds
macro_rules! try_from_lower_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl const TryFrom<$source> for $target {
            type Error = TryFromIntError;

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                if u >= 0 {
                    Ok(u as Self)
                } else {
                    Err(TryFromIntError(()))
                }
            }
        }
    )*}
}

// unsigned to signed (only positive bound)
macro_rules! try_from_upper_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl const TryFrom<$source> for $target {
            type Error = TryFromIntError;

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                if u > (Self::MAX as $source) {
                    Err(TryFromIntError(()))
                } else {
                    Ok(u as Self)
                }
            }
        }
    )*}
}

// all other cases
macro_rules! try_from_both_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl const TryFrom<$source> for $target {
            type Error = TryFromIntError;

            /// Try to create the target number type from a source
            /// number type. This returns an error if the source value
            /// is outside of the range of the target type.
            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                let min = Self::MIN as $source;
                let max = Self::MAX as $source;
                if u < min || u > max {
                    Err(TryFromIntError(()))
                } else {
                    Ok(u as Self)
                }
            }
        }
    )*}
}

macro_rules! rev {
    ($mac:ident, $source:ty, $($target:ty),*) => {$(
        $mac!($target, $source);
    )*}
}

// intra-sign conversions
try_from_upper_bounded!(u16, u8);
try_from_upper_bounded!(u32, u16, u8);
try_from_upper_bounded!(u64, u32, u16, u8);
try_from_upper_bounded!(u128, u64, u32, u16, u8);

try_from_both_bounded!(i16, i8);
try_from_both_bounded!(i32, i16, i8);
try_from_both_bounded!(i64, i32, i16, i8);
try_from_both_bounded!(i128, i64, i32, i16, i8);

// unsigned-to-signed
try_from_upper_bounded!(u8, i8);
try_from_upper_bounded!(u16, i8, i16);
try_from_upper_bounded!(u32, i8, i16, i32);
try_from_upper_bounded!(u64, i8, i16, i32, i64);
try_from_upper_bounded!(u128, i8, i16, i32, i64, i128);

// signed-to-unsigned
try_from_lower_bounded!(i8, u8, u16, u32, u64, u128);
try_from_lower_bounded!(i16, u16, u32, u64, u128);
try_from_lower_bounded!(i32, u32, u64, u128);
try_from_lower_bounded!(i64, u64, u128);
try_from_lower_bounded!(i128, u128);
try_from_both_bounded!(i16, u8);
try_from_both_bounded!(i32, u16, u8);
try_from_both_bounded!(i64, u32, u16, u8);
try_from_both_bounded!(i128, u64, u32, u16, u8);

// usize/isize
try_from_upper_bounded!(usize, isize);
try_from_lower_bounded!(isize, usize);

#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::TryFromIntError;
    use crate::convert::TryFrom;

    try_from_upper_bounded!(usize, u8);
    try_from_unbounded!(usize, u16, u32, u64, u128);
    try_from_upper_bounded!(usize, i8, i16);
    try_from_unbounded!(usize, i32, i64, i128);

    try_from_both_bounded!(isize, u8);
    try_from_lower_bounded!(isize, u16, u32, u64, u128);
    try_from_both_bounded!(isize, i8);
    try_from_unbounded!(isize, i16, i32, i64, i128);

    rev!(try_from_upper_bounded, usize, u32, u64, u128);
    rev!(try_from_lower_bounded, usize, i8, i16);
    rev!(try_from_both_bounded, usize, i32, i64, i128);

    rev!(try_from_upper_bounded, isize, u16, u32, u64, u128);
    rev!(try_from_both_bounded, isize, i32, i64, i128);
}

#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::TryFromIntError;
    use crate::convert::TryFrom;

    try_from_upper_bounded!(usize, u8, u16);
    try_from_unbounded!(usize, u32, u64, u128);
    try_from_upper_bounded!(usize, i8, i16, i32);
    try_from_unbounded!(usize, i64, i128);

    try_from_both_bounded!(isize, u8, u16);
    try_from_lower_bounded!(isize, u32, u64, u128);
    try_from_both_bounded!(isize, i8, i16);
    try_from_unbounded!(isize, i32, i64, i128);

    rev!(try_from_unbounded, usize, u32);
    rev!(try_from_upper_bounded, usize, u64, u128);
    rev!(try_from_lower_bounded, usize, i8, i16, i32);
    rev!(try_from_both_bounded, usize, i64, i128);

    rev!(try_from_unbounded, isize, u16);
    rev!(try_from_upper_bounded, isize, u32, u64, u128);
    rev!(try_from_unbounded, isize, i32);
    rev!(try_from_both_bounded, isize, i64, i128);
}

#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    use super::TryFromIntError;
    use crate::TryFrom;

    try_from_upper_bounded!(usize, u8, u16, u32);
    try_from_unbounded!(usize, u64, u128);
    try_from_upper_bounded!(usize, i8, i16, i32, i64);
    try_from_unbounded!(usize, i128);

    try_from_both_bounded!(isize, u8, u16, u32);
    try_from_lower_bounded!(isize, u64, u128);
    try_from_both_bounded!(isize, i8, i16, i32);
    try_from_unbounded!(isize, i64, i128);

    rev!(try_from_unbounded, usize, u32, u64);
    rev!(try_from_upper_bounded, usize, u128);
    rev!(try_from_lower_bounded, usize, i8, i16, i32, i64);
    rev!(try_from_both_bounded, usize, i128);

    rev!(try_from_unbounded, isize, u16, u32);
    rev!(try_from_upper_bounded, isize, u64, u128);
    rev!(try_from_unbounded, isize, i32, i64);
    rev!(try_from_both_bounded, isize, i128);
}

// Conversion traits for non-zero integer types
use core::num::NonZeroI128;
use core::num::NonZeroI16;
use core::num::NonZeroI32;
use core::num::NonZeroI64;
use core::num::NonZeroI8;
use core::num::NonZeroIsize;
use core::num::NonZeroU128;
use core::num::NonZeroU16;
use core::num::NonZeroU32;
use core::num::NonZeroU64;
use core::num::NonZeroU8;
use core::num::NonZeroUsize;

//FIXME constify this (pretty complicated)
macro_rules! nzint_impl_try_from_int {
    ($Int: ty, $NonZeroInt: ty, $doc: expr) => {
        impl TryFrom<$Int> for $NonZeroInt {
            type Error = TryFromIntError;

            // Rustdocs on the impl block show a "[+] show undocumented items" toggle.
            // Rustdocs on functions do not.
            #[doc = $doc]
            #[inline]
            fn try_from(value: $Int) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(TryFromIntError(()))
            }
        }
    };
    ($Int: ty, $NonZeroInt: ty) => {
        nzint_impl_try_from_int!(
            $Int,
            $NonZeroInt,
            concat!(
                "Attempts to convert `",
                stringify!($Int),
                "` to `",
                stringify!($NonZeroInt),
                "`."
            )
        );
    };
}

// Int -> Non-zero Int
nzint_impl_try_from_int! { u8, NonZeroU8}
nzint_impl_try_from_int! { u16, NonZeroU16}
nzint_impl_try_from_int! { u32, NonZeroU32}
nzint_impl_try_from_int! { u64, NonZeroU64}
nzint_impl_try_from_int! { u128, NonZeroU128}
nzint_impl_try_from_int! { usize, NonZeroUsize}
nzint_impl_try_from_int! { i8, NonZeroI8}
nzint_impl_try_from_int! { i16, NonZeroI16}
nzint_impl_try_from_int! { i32, NonZeroI32}
nzint_impl_try_from_int! { i64, NonZeroI64}
nzint_impl_try_from_int! { i128, NonZeroI128}
nzint_impl_try_from_int! { isize, NonZeroIsize}

//FIXME constify this (pretty complicated)
macro_rules! nzint_impl_try_from_nzint {
    ($From:ty => $To:ty, $doc: expr) => {
        impl TryFrom<$From> for $To {
            type Error = TryFromIntError;

            // Rustdocs on the impl block show a "[+] show undocumented items" toggle.
            // Rustdocs on functions do not.
            #[doc = $doc]
            #[inline]
            fn try_from(value: $From) -> Result<Self, Self::Error> {
                TryFrom::try_from(value.get()).map(|v| {
                    // SAFETY: $From is a NonZero type, so v is not zero.
                    unsafe { Self::new_unchecked(v) }
                })
            }
        }
    };
    ($To:ty: $($From: ty),*) => {$(
        nzint_impl_try_from_nzint!(
            $From => $To,
            concat!(
                "Attempts to convert `",
                stringify!($From),
                "` to `",
                stringify!($To),
                "`.",
            )
        );
    )*};
}

// Non-zero int -> non-zero unsigned int
nzint_impl_try_from_nzint! { NonZeroU8: NonZeroI8, NonZeroU16, NonZeroI16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroU16: NonZeroI8, NonZeroI16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroU32: NonZeroI8, NonZeroI16, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroU64: NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroU128: NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroUsize: NonZeroI8, NonZeroI16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroIsize }

// Non-zero int -> non-zero signed int
nzint_impl_try_from_nzint! { NonZeroI8: NonZeroU8, NonZeroU16, NonZeroI16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroI16: NonZeroU16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroI32: NonZeroU32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroI64: NonZeroU64, NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroI128: NonZeroU128, NonZeroUsize, NonZeroIsize }
nzint_impl_try_from_nzint! { NonZeroIsize: NonZeroU16, NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64, NonZeroU128, NonZeroI128, NonZeroUsize }
