use std::collections::HashMap;
use std::hash::Hash;
use std::result::Result;
use std::fmt;
use std::rc::Rc;

pub trait Id: Clone + Eq + Ord + Sized + Hash + VariableDisplay {}

impl<T> Id for T where T: Clone + Eq + Ord + Sized + Hash + VariableDisplay {}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial<I> where I: Id {
    pub coefficient : i64,
    pub powers : Vec<(Composite<I>, u8)>
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial<I> where I: Id {
    pub monomials: Vec<Monomial<I>>
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Composite<I> where I: Id {
    Variable(I),
    Floor(Rc<Polynomial<I>>, Rc<Polynomial<I>>),
    Ceil(Rc<Polynomial<I>>, Rc<Polynomial<I>>),
    Min(Rc<Polynomial<I>>, Rc<Polynomial<I>>),
    Max(Rc<Polynomial<I>>, Rc<Polynomial<I>>)
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
/// let a = primitive(0);
/// let b = primitive(1);
/// let a_square_plus_b_plus_1 = &(&a * &a) + &(&b + 1);
/// let mut values: HashMap<u16, i64> = HashMap::new();
/// values.insert(0, 20);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Err(1));
/// values.insert(1, 3);
/// assert!(a_square_plus_b_plus_1.evaluate(&values) == Ok(404));
/// ```
pub trait Evaluable<I> where I: Id {
    fn evaluate(&self, values: &HashMap<I, i64>) -> Result<i64, I>;
}

/// A common trait for expressions which can be constants.
///
/// # Examples
/// ```
/// # use symints::*;
/// let a = primitive(0);
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
/// let a = primitive(0);
/// let b = primitive(1);
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
pub fn primitive<I>(id: I) -> Polynomial<I> where I: Id {
    Polynomial{
        monomials: vec![Monomial{
            coefficient: 1,
            powers: vec![(Composite::Variable(id), 1)]
        }]
    }
}

/// Computes a symbolic `max` between two symbolic integer expressions.
pub fn max<I>(left: &Polynomial<I>, right: &Polynomial<I>) -> Polynomial<I> where I: Id {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        Polynomial::from( if v1 > v2 {v1} else {v2})
    } else {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: 1,
                powers: vec![(Composite::Max(Rc::new(left.clone()),
                                             Rc::new(right.clone())), 1)]
            }]
        }
    }
}

/// Computes a symbolic `min` between two symbolic integer expressions.
pub fn min<I>(left: &Polynomial<I>, right: &Polynomial<I>) -> Polynomial<I> where I: Id {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap();
        Polynomial::from( if v1 < v2 {v1} else {v2})
    } else {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: 1,
                powers: vec![(Composite::Min(Rc::new(left.clone()),
                                             Rc::new(right.clone())), 1)]
            }]
        }
    }
}

/// Computes a symbolic `ceil` between two symbolic integer expressions.
pub fn ceil<I>(left: &Polynomial<I>, right: &Polynomial<I>) -> Polynomial<I> where I: Id {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap() as f64;
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap() as f64;
        Polynomial::from( (v1 / v2).ceil() as i64)
    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => Polynomial {
                monomials: vec![Monomial {
                    coefficient: 1,
                    powers: vec![(Composite::Ceil(Rc::new(left.clone()),
                                                  Rc::new(right.clone())), 1)]
                }]
            }
        }
    }
}

/// Computes a symbolic `floor` between two symbolic integer expressions.
pub fn floor<I>(left: &Polynomial<I>, right: &Polynomial<I>) -> Polynomial<I> where I: Id {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).ok().unwrap() as f64;
        let v2 = right.evaluate(&HashMap::default()).ok().unwrap() as f64;
        Polynomial::from( (v1 / v2).floor() as i64)
    } else {
        match left.checked_div(right) {
            Some(polynomial) => polynomial,
            None => Polynomial {
                monomials: vec![Monomial {
                    coefficient: 1,
                    powers: vec![(Composite::Floor(Rc::new(left.clone()),
                                                   Rc::new(right.clone())), 1)]
                }]
            }
        }
    }
}