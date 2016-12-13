use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use std::collections::HashMap;
use std::hash::Hash;
use std::result::Result;
use std::fmt;
use std::rc::Rc;

use num::{Integer, One};

pub trait Id: Clone + Ord + Hash + VariableDisplay {}

impl<T> Id for T where T: Clone + Ord + Hash + VariableDisplay {}

pub trait Power: Clone + Ord + fmt::Display + fmt::Debug {}

impl<T> Power for T where T: Clone + Ord + fmt::Display + fmt::Debug {}

pub trait Coefficient: Integer + One +
    AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + Neg<Output=Self> +
    Clone + fmt::Display + fmt::Debug {}

impl<T> Coefficient for T where T: Integer + One +
    AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + Neg<Output=T> +
    Clone + fmt::Display + fmt::Debug {}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial<I, C> where I: Id, C: Coefficient {
    pub coefficient : C,
    pub powers : Vec<(Composite<I, C>, u8)>
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial<I, C> where I: Id, C: Coefficient {
    pub monomials: Vec<Monomial<I, C>>
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Composite<I, C> where I: Id, C: Coefficient {
    Variable(I),
    Floor(Rc<Polynomial<I, C>>, Rc<Polynomial<I, C>>),
    Ceil(Rc<Polynomial<I, C>>, Rc<Polynomial<I, C>>),
    Min(Rc<Polynomial<I, C>>, Rc<Polynomial<I, C>>),
    Max(Rc<Polynomial<I, C>>, Rc<Polynomial<I, C>>)
}

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
/// type SymInt = Polynomial<u16, i64>;
/// let a: SymInt = primitive(0);
/// let b: SymInt = primitive(1);
/// let a_square_plus_b_plus_1 = &(&a * &a) + &(&b + 1);
/// let mut values: HashMap<u16, i64> = HashMap::new();
/// values.insert(0, 20);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Err(1));
/// values.insert(1, 3);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Ok(404));
/// ```
pub trait Evaluable<I, C> where I: Id, C: Coefficient {
    fn evaluate(&self, values: &HashMap<I, C>) -> Result<C, I>;
}

/// A common trait for expressions which can be constants.
///
/// # Examples
/// ```
/// # use symints::*;
/// type SymInt = Polynomial<u16, i64>;
/// let a: SymInt = primitive(0);
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
/// type SymInt = Polynomial<u16, i64>;
/// let a: SymInt = primitive(0);
/// let b: SymInt = primitive(1);
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
    fn var_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
}

/// Returns a symbolic integer expression representing
/// the primitive variable `Composite::Variable(id)`.
pub fn primitive<I, C>(id: I) -> Polynomial<I, C> where I: Id, C: Coefficient {
    Polynomial{
        monomials: vec![Monomial{
            coefficient: C::one(),
            powers: vec![(Composite::Variable(id), 1)]
        }]
    }
}

/// Computes a symbolic `max` between two symbolic integer expressions.
pub fn max<I, C>(left: &Polynomial<I, C>, right: &Polynomial<I, C>) -> Polynomial<I, C>
    where I: Id, C: Coefficient {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        Polynomial::from( if v1 > v2 {v1} else {v2})
    } else {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: C::one(),
                powers: vec![(Composite::Max(Rc::new(left.clone()),
                                             Rc::new(right.clone())), 1)]
            }]
        }
    }
}

/// Computes a symbolic `min` between two symbolic integer expressions.
pub fn min<I, C>(left: &Polynomial<I, C>, right: &Polynomial<I, C>) -> Polynomial<I, C>
    where I: Id, C: Coefficient {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        Polynomial::from( if v1 < v2 {v1} else {v2})
    } else {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: C::one(),
                powers: vec![(Composite::Min(Rc::new(left.clone()),
                                             Rc::new(right.clone())), 1)]
            }]
        }
    }
}

/// Computes a symbolic `ceil` between two symbolic integer expressions.
pub fn ceil<I, C>(left: &Polynomial<I, C>, right: &Polynomial<I, C>) -> Polynomial<I, C>
    where I: Id, C: Coefficient {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        let (d, rem) = v1.div_rem(&v2);
        if rem == C::zero() {
            Polynomial::from(d)
        } else {
            Polynomial::from(d + C::one())
        }

    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => Polynomial {
                monomials: vec![Monomial {
                    coefficient: C::one(),
                    powers: vec![(Composite::Ceil(Rc::new(left.clone()),
                                                  Rc::new(right.clone())), 1)]
                }]
            }
        }
    }
}

/// Computes a symbolic `floor` between two symbolic integer expressions.
pub fn floor<I, C>(left: &Polynomial<I, C>, right: &Polynomial<I, C>) -> Polynomial<I, C>
    where I: Id, C: Coefficient {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        Polynomial::from(C::div_floor(&v1, &v2))
    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => Polynomial {
                monomials: vec![Monomial {
                    coefficient: C::one(),
                    powers: vec![(Composite::Floor(Rc::new(left.clone()),
                                                   Rc::new(right.clone())), 1)]
                }]
            }
        }
    }
}