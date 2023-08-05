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

impl<T, U> Into<U> for T
where
    U: From<T>,
{
    #[inline]
    fn into(self) -> U {
        U::from(self)
    }
}

impl<T> From<T> for T {
    /// Returns the argument unchanged.
    #[inline(always)]
    fn from(t: T) -> T {
        t
    }
}

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
{
    type Error = Infallible;

    #[inline]
    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::into(value))
    }
}
