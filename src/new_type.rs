use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::ops::{Add, Deref, Sub};
use std::str::FromStr;

pub trait NewType<T>:
    FromStr
    + Display
    + From<T>
    + Into<T>
    + AsRef<T>
    + Borrow<T>
    + Deref<Target = T>
    + Debug
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Copy
    + Clone
    + Add<Output = Self>
    + Add<T, Output = Self>
    + Sub<Output = Self>
    + Sub<T, Output = Self>
{
}

#[macro_export]
macro_rules! new_type {
    ( $( $vis:vis struct $name:ident ( $ty:ty ); )+ ) => {
        $(
            #[derive(shrinkwraprs::Shrinkwrap, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
            $vis struct $name ( $ty );

            impl std::str::FromStr for $name {
                type Err = <$ty as std::str::FromStr>::Err;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    <$ty as std::str::FromStr>::from_str(s).map($name)
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    $name(value)
                }
            }

            impl From<$name> for $ty {
                fn from(value: $name) -> Self {
                    value.0
                }
            }

            impl std::ops::Add<$name> for $name {
                type Output = $name;
                fn add(self, rhs: $name) -> Self::Output {
                    $name(self.0 + rhs.0)
                }
            }

            impl std::ops::Add<$ty> for $name {
                type Output = $name;
                fn add(self, rhs: $ty) -> Self::Output {
                    $name(self.0 + rhs)
                }
            }

            impl std::ops::Sub<$name> for $name {
                type Output = $name;
                fn sub(self, rhs: $name) -> Self::Output {
                    $name(self.0 - rhs.0)
                }
            }

            impl std::ops::Sub<$ty> for $name {
                type Output = $name;
                fn sub(self, rhs: $ty) -> Self::Output {
                    $name(self.0 - rhs)
                }
            }

            impl $crate::new_type::NewType<$ty> for $name {

            }
        )+
    }
}
