/// A trait for symbolic integer expressions which can represent constants.
///
/// # Examples
/// ```
/// # use symints::*;
/// let mut registry = Registry::default();
/// let a = registry.new_monomial_variable();
/// let zero = &a - &a;
/// assert!(!a.is_constant());
/// assert!(zero.is_constant());
/// ```
pub trait IsConstant {
    fn is_constant(&self) -> bool;
}

/// A trait for checked division of symbolic integer expressions.
///
/// Computes `self / other`, returning `None` if `other == 0` or
/// if the symbolic expression in `self` is not divisible by `other`.
///
/// # Examples
/// ```
/// # use symints::*;
/// let mut registry = Registry::default();
/// let a = registry.new_monomial_variable();
/// let b = registry.new_monomial_variable();
/// let a_plus_b = &a + &b;
/// let a_plus_b_square = &a_plus_b * &a_plus_b;
/// assert!(a_plus_b_square.checked_div(&a_plus_b) == Some(a_plus_b));
/// assert!(a_plus_b_square.checked_div(&a).is_none());
/// ```
pub trait CheckedDiv<RHS = Self> {
    type Output;
    fn checked_div(&self, other: RHS) -> Option<Self::Output>;
}


/// A trait for objects which can calculate floor for types `L` and `R`.
pub trait Floor<L, R> {
    type Output;
    fn floor(&mut self, left: L, right: R) -> Self::Output;
}

/// A trait for objects which can calculate floor for types `L` and `R`.
pub trait Ceil<L, R> {
    type Output;
    fn ceil(&mut self, left: L, right: R) -> Self::Output;
}

/// A trait for objects which can calculate floor for types `L` and `R`.
pub trait Min<L, R> {
    type Output;
    fn min(&mut self, left: L, right: R) -> Self::Output;
}

/// A trait for objects which can calculate floor for types `L` and `R`.
pub trait Max<L, R> {
    type Output;
    fn max(&mut self, left: L, right: R) -> Self::Output;
}
