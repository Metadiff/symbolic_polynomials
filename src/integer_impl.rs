use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{Ordering};
use std::fmt;
use std::convert::From;

use primitives::*;

impl VariableDisplay for u16 {
    fn var_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", (*self as u8 + ('a' as u8)) as char)
    }
}

impl<I> PartialEq<Monomial<I>> for i64 where I: Id {
    fn eq(&self, other: &Monomial<I>) -> bool {
        other.eq(self)
    }
}

impl<I> PartialEq<Polynomial<I>> for i64 where I: Id {
    fn eq(&self, other: &Polynomial<I>) -> bool {
        other.eq(self)
    }
}

impl<I> PartialOrd<Monomial<I>> for i64 where I: Id {
    fn partial_cmp(&self, other: &Monomial<I>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<I> PartialOrd<Polynomial<I>> for i64 where I: Id {
    fn partial_cmp(&self, other: &Polynomial<I>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<'a, I> Div<&'a Monomial<I>> for i64 where I: Id {
    type Output = Option<Monomial<I>>;
    fn div(self, rhs: &'a Monomial<I>) -> Self::Output {
        if rhs.is_constant() {
            match self.checked_rem(rhs.coefficient) {
                Some(0) => Some(Monomial::from(self / rhs.coefficient)),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'a, I> Div<&'a Polynomial<I>> for i64 where I: Id {
    type Output = Option<Polynomial<I>>;
    fn div(self, rhs: &'a Polynomial<I>) -> Self::Output {
        match rhs.monomials.len() {
            1 => match self / &(rhs.monomials[0]) {
                Some(m) => Some(Polynomial{monomials: vec![m]}),
                None => None
            },
            _ => None
        }
    }
}

impl<'a, I> Mul<&'a Monomial<I>> for i64 where I: Id {
    type Output = Monomial<I>;
    fn mul(self, rhs: &'a Monomial<I>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a, I> Mul<&'a Polynomial<I>> for i64 where I: Id {
    type Output = Polynomial<I>;
    fn mul(self, rhs: &'a Polynomial<I>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a, I> Add<&'a Monomial<I>> for i64 where I: Id {
    type Output = Polynomial<I>;
    fn add(self, rhs: &'a Monomial<I>) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a, I> Sub<&'a Monomial<I>> for i64 where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'a Monomial<I>) -> Self::Output {
        -&(rhs.add(-self))
    }
}

impl<'a, I> Sub<&'a Polynomial<I>> for i64 where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'a Polynomial<I>) -> Self::Output {
        -&(rhs.add(-self))
    }
}
