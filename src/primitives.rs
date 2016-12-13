use std::collections::HashMap;
use std::result::Result;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Constraint{
    Positive,
    Negative,
    Unknown
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial {
    pub coefficient : i64,
    pub powers : Vec<(Composite, u8)>
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial {
    pub monomials: Vec<Monomial>
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Composite{
    Variable(u16, Constraint),
    Floor(Rc<Polynomial>, Rc<Polynomial>),
    Ceil(Rc<Polynomial>, Rc<Polynomial>),
    Min(Rc<Polynomial>, Rc<Polynomial>),
    Max(Rc<Polynomial>, Rc<Polynomial>)
}

/// A trait for symbolic integer expressions which can be evaluated.
///
/// The evaluation takes as an extra arguments a mapping between a variable representation
/// of Composite::Variable and the actual numeric value to be assigned to it.
///
pub trait Evaluable {
    fn evaluate(&self, values: &HashMap<u16, i64>) -> Result<i64, u16>;
}

/// A trait for symbolic integer expressions which can represent constants.
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

/// A trait for checked division of symbolic integer expressions.
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

pub fn primitive(id: u16) -> Polynomial {
    Polynomial{
        monomials: vec![Monomial{
            coefficient: 1,
            powers: vec![(Composite::Variable(id, Constraint::Unknown), 1)]
        }]
    }
}

pub fn constraint_primitive(id: u16, c: Constraint) -> Polynomial {
    Polynomial{
        monomials: vec![Monomial{
            coefficient: 1,
            powers: vec![(Composite::Variable(id, c), 1)]
        }]
    }
}

pub fn max(left: &Polynomial, right: &Polynomial) -> Polynomial {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).unwrap();
        let v2 = right.evaluate(&HashMap::default()).unwrap();
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

pub fn min(left: &Polynomial, right: &Polynomial) -> Polynomial {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).unwrap();
        let v2 = right.evaluate(&HashMap::default()).unwrap();
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

pub fn ceil(left: &Polynomial, right: &Polynomial) -> Polynomial {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).unwrap() as f64;
        let v2 = right.evaluate(&HashMap::default()).unwrap() as f64;
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

pub fn floor(left: &Polynomial, right: &Polynomial) -> Polynomial {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&HashMap::default()).unwrap() as f64;
        let v2 = right.evaluate(&HashMap::default()).unwrap() as f64;
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