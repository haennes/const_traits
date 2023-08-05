#![feature(const_trait_impl, never_type)]
use core::convert::Infallible;

mod impls;
pub use const_ops::*;

#[const_trait]
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}
#[const_trait]
pub trait Into<T>: Sized {
    fn into(self) -> T;
}
#[const_trait]
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
#[const_trait]
pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

impl<T, U> const Into<U> for T
where
    U: ~const From<T>,
{
    #[inline]
    fn into(self) -> U {
        U::from(self)
    }
}

impl<T> const From<T> for T {
    /// Returns the argument unchanged.
    #[inline(always)]
    fn from(t: T) -> T {
        t
    }
}

impl<T, U> const TryInto<U> for T
where
    U: ~const TryFrom<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

impl<T, U> const TryFrom<U> for T
where
    U: ~const Into<T>,
{
    type Error = Infallible;

    #[inline]
    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::into(value))
    }
}
