use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use num::{Integer, One, Zero, Unsigned};

/// A trait specifying all the bounds an `Id` type should meet.
///
/// See the module-level documentation for more.
pub trait Id: Clone + Ord + Default + ::std::hash::Hash + ::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Id for T where T: Clone + Ord + Default + ::std::hash::Hash + ::std::fmt::Display + ::std::fmt::Debug {}

/// A trait specifying all the bounds a `Power` type should meet.
///
/// See the module-level documentation for more.
pub trait Power: Integer + One + Zero + Unsigned + ::num::ToPrimitive + ::num::FromPrimitive + Clone + Ord +
    ::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Power for T
    where T: Integer + One + Zero + Unsigned + ::num::ToPrimitive + ::num::FromPrimitive + Clone + Ord +
    ::std::fmt::Display + ::std::fmt::Debug {}

/// A trait specifying all the bounds a `Coefficient` type should meet.
///
/// See the module-level documentation for more.
pub trait Coefficient: Integer + One + ::num::ToPrimitive + ::num::FromPrimitive +
AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + Neg<Output=Self> +
Clone + ::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Coefficient for T where T: Integer + One + ::num::ToPrimitive + ::num::FromPrimitive +
AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + Neg<Output=T> +
Clone + ::std::fmt::Display + ::std::fmt::Debug {}

