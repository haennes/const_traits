use crate::From;

// Conversion traits for primitive integer and float types
// Conversions T -> T are covered by a blanket impl and therefore excluded
// Some conversions from and to usize/isize are not implemented due to portability concerns
macro_rules! impl_from {
    ($Small: ty, $Large: ty, $doc: expr) => {
        impl const From<$Small> for $Large {
            // Rustdocs on the impl block show a "[+] show undocumented items" toggle.
            // Rustdocs on functions do not.
            #[doc = $doc]
            #[inline(always)]
            fn from(small: $Small) -> Self {
                small as Self
            }
        }
    };
    ($Small: ty, $Large: ty) => {
        impl_from!(
            $Small,
            $Large,
            concat!(
                "Converts `",
                stringify!($Small),
                "` to `",
                stringify!($Large),
                "` losslessly."
            )
        );
    };
}

macro_rules! impl_from_bool {
    ($target: ty) => {
        impl_from!(
            bool,
            $target,
            concat!(
                "Converts a `bool` to a `",
                stringify!($target),
                "`. The resulting value is `0` for `false` and `1` for `true`
values.

# Examples

```
assert_eq!(",
                stringify!($target),
                "::from(true), 1);
assert_eq!(",
                stringify!($target),
                "::from(false), 0);
```"
            )
        );
    };
}

// Bool -> Any
impl_from_bool! { u8}
impl_from_bool! { u16}
impl_from_bool! { u32}
impl_from_bool! { u64}
impl_from_bool! { u128}
impl_from_bool! { usize}
impl_from_bool! { i8}
impl_from_bool! { i16}
impl_from_bool! { i32}
impl_from_bool! { i64}
impl_from_bool! { i128}
impl_from_bool! { isize}

// Unsigned -> Unsigned
impl_from! { u8, u16}
impl_from! { u8, u32}
impl_from! { u8, u64}
impl_from! { u8, u128}
impl_from! { u8, usize}
impl_from! { u16, u32}
impl_from! { u16, u64}
impl_from! { u16, u128}
impl_from! { u32, u64}
impl_from! { u32, u128}
impl_from! { u64, u128}

// Signed -> Signed
impl_from! { i8, i16}
impl_from! { i8, i32}
impl_from! { i8, i64}
impl_from! { i8, i128}
impl_from! { i8, isize}
impl_from! { i16, i32}
impl_from! { i16, i64}
impl_from! { i16, i128}
impl_from! { i32, i64}
impl_from! { i32, i128}
impl_from! { i64, i128}

// Unsigned -> Signed
impl_from! { u8, i16}
impl_from! { u8, i32}
impl_from! { u8, i64}
impl_from! { u8, i128}
impl_from! { u16, i32}
impl_from! { u16, i64}
impl_from! { u16, i128}
impl_from! { u32, i64}
impl_from! { u32, i128}
impl_from! { u64, i128}

// The C99 standard defines bounds on INTPTR_MIN, INTPTR_MAX, and UINTPTR_MAX
// which imply that pointer-sized integers must be at least 16 bits:
// https://port70.net/~nsz/c/c99/n1256.html#7.18.2.4
impl_from! { u16, usize}
impl_from! { u8, isize}
impl_from! { i16, isize}

// RISC-V defines the possibility of a 128-bit address space (RV128).

// CHERI proposes 256-bit “capabilities”. Unclear if this would be relevant to usize/isize.
// https://www.cl.cam.ac.uk/research/security/ctsrd/pdfs/20171017a-cheri-poster.pdf
// https://www.csl.sri.com/users/neumann/2012resolve-cheri.pdf

// Note: integers can only be represented with full precision in a float if
// they fit in the significand, which is 24 bits in f32 and 53 bits in f64.
// Lossy float conversions are not implemented at this time.

// Signed -> Float
impl_from! { i8, f32}
impl_from! { i8, f64}
impl_from! { i16, f32}
impl_from! { i16, f64}
impl_from! { i32, f64}

// Unsigned -> Float
impl_from! { u8, f32}
impl_from! { u8, f64}
impl_from! { u16, f32}
impl_from! { u16, f64}
impl_from! { u32, f64}

// Float -> Float
impl_from! { f32, f64}

// bool -> Float
impl const From<bool> for f32 {
    /// Converts `bool` to `f32` losslessly. The resulting value is positive
    /// `0.0` for `false` and `1.0` for `true` values.
    ///
    /// # Examples
    /// ```
    /// let x: f32 = false.into();
    /// assert_eq!(x, 0.0);
    /// assert!(x.is_sign_positive());
    ///
    /// let y: f32 = true.into();
    /// assert_eq!(y, 1.0);
    /// ```
    #[inline]
    fn from(small: bool) -> Self {
        small as u8 as Self
    }
}

impl const From<bool> for f64 {
    /// Converts `bool` to `f64` losslessly. The resulting value is positive
    /// `0.0` for `false` and `1.0` for `true` values.
    ///
    /// # Examples
    /// ```
    /// let x: f64 = false.into();
    /// assert_eq!(x, 0.0);
    /// assert!(x.is_sign_positive());
    ///
    /// let y: f64 = true.into();
    /// assert_eq!(y, 1.0);
    /// ```
    #[inline]
    fn from(small: bool) -> Self {
        small as u8 as Self
    }
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

macro_rules! nzint_impl_from {
    ($Small: ty, $Large: ty, $doc: expr) => {
        impl const From<$Small> for $Large {
            // Rustdocs on the impl block show a "[+] show undocumented items" toggle.
            // Rustdocs on functions do not.
            #[doc = $doc]
            #[inline]
            fn from(small: $Small) -> Self {
                // SAFETY: input type guarantees the value is non-zero
                unsafe { Self::new_unchecked(From::from(small.get())) }
            }
        }
    };
    ($Small: ty, $Large: ty) => {
        nzint_impl_from!(
            $Small,
            $Large,
            concat!(
                "Converts `",
                stringify!($Small),
                "` to `",
                stringify!($Large),
                "` losslessly."
            )
        );
    };
}

// Non-zero Unsigned -> Non-zero Unsigned
nzint_impl_from! { NonZeroU8, NonZeroU16}
nzint_impl_from! { NonZeroU8, NonZeroU32}
nzint_impl_from! { NonZeroU8, NonZeroU64}
nzint_impl_from! { NonZeroU8, NonZeroU128}
nzint_impl_from! { NonZeroU8, NonZeroUsize}
nzint_impl_from! { NonZeroU16, NonZeroU32}
nzint_impl_from! { NonZeroU16, NonZeroU64}
nzint_impl_from! { NonZeroU16, NonZeroU128}
nzint_impl_from! { NonZeroU16, NonZeroUsize}
nzint_impl_from! { NonZeroU32, NonZeroU64}
nzint_impl_from! { NonZeroU32, NonZeroU128}
nzint_impl_from! { NonZeroU64, NonZeroU128}

// Non-zero Signed -> Non-zero Signed
nzint_impl_from! { NonZeroI8, NonZeroI16}
nzint_impl_from! { NonZeroI8, NonZeroI32}
nzint_impl_from! { NonZeroI8, NonZeroI64}
nzint_impl_from! { NonZeroI8, NonZeroI128}
nzint_impl_from! { NonZeroI8, NonZeroIsize}
nzint_impl_from! { NonZeroI16, NonZeroI32}
nzint_impl_from! { NonZeroI16, NonZeroI64}
nzint_impl_from! { NonZeroI16, NonZeroI128}
nzint_impl_from! { NonZeroI16, NonZeroIsize}
nzint_impl_from! { NonZeroI32, NonZeroI64}
nzint_impl_from! { NonZeroI32, NonZeroI128}
nzint_impl_from! { NonZeroI64, NonZeroI128}

// NonZero UnSigned -> Non-zero Signed
nzint_impl_from! { NonZeroU8, NonZeroI16}
nzint_impl_from! { NonZeroU8, NonZeroI32}
nzint_impl_from! { NonZeroU8, NonZeroI64}
nzint_impl_from! { NonZeroU8, NonZeroI128}
nzint_impl_from! { NonZeroU8, NonZeroIsize}
nzint_impl_from! { NonZeroU16, NonZeroI32}
nzint_impl_from! { NonZeroU16, NonZeroI64}
nzint_impl_from! { NonZeroU16, NonZeroI128}
nzint_impl_from! { NonZeroU32, NonZeroI64}
nzint_impl_from! { NonZeroU32, NonZeroI128}
nzint_impl_from! { NonZeroU64, NonZeroI128}
