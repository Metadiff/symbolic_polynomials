use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use num::{Integer, Unsigned, Bounded, ToPrimitive, FromPrimitive,
          CheckedMul, CheckedDiv, CheckedAdd, CheckedSub};
use num::traits::NumAssign;
use std::collections::HashMap;

/// A trait specifying all the bounds an `Id` type should meet.
///
/// See the module-level documentation for more.
pub trait Id
    : Clone + Ord + Default + ::std::hash::Hash + ::std::fmt::Display + ::std::fmt::Debug
    {
}

impl<T> Id for T
    where T: Clone + Ord + Default + ::std::hash::Hash + ::std::fmt::Display + ::std::fmt::Debug {
}

/// A trait specifying all the bounds a `Power` type should meet.
///
/// See the module-level documentation for more.
pub trait Power
    : Integer
    + Unsigned
    + ToPrimitive
    + FromPrimitive
    + Clone
    + Ord
    + ::std::fmt::Display
    + ::std::fmt::Debug {
}

impl<T> Power for T
    where T: Integer
                 + Unsigned
                 + ToPrimitive
                 + FromPrimitive
                 + Clone
                 + Ord
                 + ::std::fmt::Display
                 + ::std::fmt::Debug {
}

/// A trait specifying all the bounds a `Coefficient` type should meet.
///
/// See the module-level documentation for more.
pub trait Coefficient
    : Integer
    + ToPrimitive
    + FromPrimitive
    + Bounded
    + NumAssign
    + Neg<Output = Self>
    + CheckedMul
    + CheckedAdd
    + CheckedSub
    + Clone
    + ::std::fmt::Display
    + ::std::fmt::Debug {
}

impl<T> Coefficient for T
    where T: Integer
                 + ToPrimitive
                 + FromPrimitive
                 + Bounded
                 + NumAssign
                 + Neg<Output = T>
                 + CheckedMul
                 + CheckedAdd
                 + CheckedSub
                 + Clone
                 + ::std::fmt::Display
                 + ::std::fmt::Debug {
}


pub trait Evaluable<I, C> {
    /// Evaluates the given the provided mapping of identifiers to value assignments.
    fn eval(&self, values: &HashMap<I, C>) -> Result<C, (I, String)>;
}