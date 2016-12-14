use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use num::{Integer, One, Zero, Unsigned};

use monomial::Monomial;
use polynomial::Polynomial;
use composite::Composite;

/// A trait specifying all the bounds an `Id` type should meet.
///
/// See the module-level documentation for more.
pub trait Id
    : Clone + Ord + ::std::hash::Hash + VariableDisplay + ::std::fmt::Debug {
}

impl<T> Id for T where T: Clone + Ord + ::std::hash::Hash + VariableDisplay + ::std::fmt::Debug {}

/// A trait specifying all the bounds a `Power` type should meet.
///
/// See the module-level documentation for more.
pub trait Power
    : Integer + One + Zero + Unsigned + Into<usize> + Clone + Ord + ::std::fmt::Display + ::std::fmt::Debug
    {
}

impl<T> Power for T
    where T: Integer + One + Zero + Unsigned + Into<usize> + Clone + Ord + ::std::fmt::Display + ::std::fmt::Debug {
}

/// A trait specifying all the bounds a `Coefficient` type should meet.
///
/// See the module-level documentation for more.
pub trait Coefficient: Integer + One +
AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + Neg<Output=Self> +
Clone + ::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Coefficient for T where T: Integer + One +
AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + Neg<Output=T> +
Clone + ::std::fmt::Display + ::std::fmt::Debug {}

/// A common trait for expressions which can be evaluated.
///
/// Evaluation is done via a mapping between a variables representation
/// (e.g. `I` for `Composite::Variable(I)`)
/// and the numeric values to be assigned to them.
///
/// If there are expressions which require a variable which has not been assigned a value
/// an `Err` with the first such variable is returned.
///
/// # Examples
/// ```
/// # use symints::*;
/// # use std::collections::HashMap;
/// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// struct VarId{
///     pub id: u16
/// }
///
/// impl From<u16> for VarId {
///     fn from(other: u16) -> Self {
///         VarId{id: other}
///     }
/// }
///
/// impl VariableDisplay for VarId {
///     fn var_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>{
///         write!(f, "{}", (self.id as u8+ ('a' as u8)) as char)
///     }
/// }
///
/// type SymInt = Polynomial<VarId, i64, u8>;
/// let a: SymInt = primitive(0.into());
/// let b: SymInt = primitive(1.into());
/// let a_square_plus_b_plus_1 = &(&a * &a) + &(&b + 1);
/// let mut values: HashMap<VarId, i64> = HashMap::new();
/// values.insert(0.into(), 20);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Err(1.into()));
/// values.insert(1.into(), 3);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Ok(404));
/// ```
pub trait Evaluable<I, C>
    where I: Id, C: Coefficient {
    fn evaluate(&self, values: &::std::collections::HashMap<I, C>) -> ::std::result::Result<C, I>;
}

/// A common trait for expressions which can be constants.
///
/// # Examples
/// ```
/// # use symints::*;
/// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// struct VarId{
///     pub id: u16
/// }
///
/// impl From<u16> for VarId {
///     fn from(other: u16) -> Self {
///         VarId{id: other}
///     }
/// }
///
/// impl VariableDisplay for VarId {
///     fn var_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>{
///         write!(f, "{}", (self.id as u8+ ('a' as u8)) as char)
///     }
/// }
///
/// type SymInt = Polynomial<VarId, i64, u8>;
/// let a: SymInt = primitive(0.into());
/// let zero = &a - &a;
/// assert!(!a.is_constant());
/// assert!(zero.is_constant());
/// ```
pub trait IsConstant {
    fn is_constant(&self) -> bool;
}

/// A common trait for expressions which can perform checked division.
///
/// Computes `self / other`, returning `None` if `other == 0` or
/// if the symbolic expression in `self` is not divisible by `other`.
///
/// # Examples
/// ```
/// # use symints::*;
/// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// struct VarId{
///     pub id: u16
/// }
///
/// impl From<u16> for VarId {
///     fn from(other: u16) -> Self {
///         VarId{id: other}
///     }
/// }
///
/// impl VariableDisplay for VarId {
///     fn var_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>{
///         write!(f, "{}", (self.id as u8+ ('a' as u8)) as char)
///     }
/// }
///
/// type SymInt = Polynomial<VarId, i64, u8>;
/// let a: SymInt = primitive(0.into());
/// let b: SymInt = primitive(1.into());
/// let a_plus_b = &a + &b;
/// let a_plus_b_square = &a_plus_b * &a_plus_b;
/// assert!(a_plus_b_square.checked_div(&a_plus_b) == Some(a_plus_b));
/// assert!(a_plus_b_square.checked_div(&a).is_none());
/// ```
pub trait CheckedDiv<RHS = Self> {
    type Output;
    fn checked_div(&self, other: RHS) -> Option<Self::Output>;
}

pub trait VariableDisplay {
    fn var_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error>;
}

/// Returns a symbolic integer expression representing
/// the primitive variable `Composite::Variable(id)`.
pub fn primitive<I, C, P>(id: I) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    Polynomial {
        monomials: vec![Monomial {
                            coefficient: C::one(),
                            powers: vec![(Composite::Variable(id), P::one())],
                        }],
    }
}

/// Computes a symbolic `max` between two symbolic integer expressions.
pub fn max<I, C, P>(left: &Polynomial<I, C, P>, right: &Polynomial<I, C, P>) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        Polynomial::from(if v1 > v2 { v1 } else { v2 })
    } else {
        Polynomial {
            monomials: vec![Monomial {
                                coefficient: C::one(),
                                powers: vec![(Composite::Max(::std::rc::Rc::new(left.clone()),
                                                             ::std::rc::Rc::new(right.clone())),
                                              P::one())],
                            }],
        }
    }
}

/// Computes a symbolic `min` between two symbolic integer expressions.
pub fn min<I, C, P>(left: &Polynomial<I, C, P>, right: &Polynomial<I, C, P>) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        Polynomial::from(if v1 < v2 { v1 } else { v2 })
    } else {
        Polynomial {
            monomials: vec![Monomial {
                                coefficient: C::one(),
                                powers: vec![(Composite::Min(::std::rc::Rc::new(left.clone()),
                                                             ::std::rc::Rc::new(right.clone())),
                                              P::one())],
                            }],
        }
    }
}

/// Computes a symbolic `ceil` between two symbolic integer expressions.
pub fn ceil<I, C, P>(left: &Polynomial<I, C, P>, right: &Polynomial<I, C, P>) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let (d, rem) = v1.div_rem(&v2);
        if rem == C::zero() {
            Polynomial::from(d)
        } else {
            Polynomial::from(d + C::one())
        }
    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => {
                Polynomial {
                    monomials: vec![Monomial {
                                        coefficient: C::one(),
                                        powers: vec![(Composite::Ceil(::std::rc::Rc::new(left.clone()),
                                                                      ::std::rc::Rc::new(right.clone())),
                                                      P::one())],
                                    }],
                }
            }
        }
    }
}

/// Computes a symbolic `floor` between two symbolic integer expressions.
pub fn floor<I, C, P>(left: &Polynomial<I, C, P>, right: &Polynomial<I, C, P>) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        Polynomial::from(C::div_floor(&v1, &v2))
    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => {
                Polynomial {
                    monomials: vec![Monomial {
                                        coefficient: C::one(),
                                        powers: vec![(Composite::Floor(::std::rc::Rc::new(left.clone()),
                                                                       ::std::rc::Rc::new(right.clone())),
                                                      P::one())],
                                    }],
                }
            }
        }
    }
}
