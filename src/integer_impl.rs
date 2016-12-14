use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;

impl<I, P> PartialEq<Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    fn eq(&self, other: &Monomial<I, i64, P>) -> bool {
        other.eq(self)
    }
}

impl<I, P> PartialEq<Polynomial<I, i64, P>> for i64
    where I: Id, P: Power {
    fn eq(&self, other: &Polynomial<I, i64, P>) -> bool {
        other.eq(self)
    }
}

impl<I, P> PartialOrd<Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    fn partial_cmp(&self, other: &Monomial<I, i64, P>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None,
        }
    }
}

impl<I, P> PartialOrd<Polynomial<I, i64, P>> for i64
    where I: Id, P: Power {
    fn partial_cmp(&self, other: &Polynomial<I, i64, P>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None,
        }
    }
}

impl<'a, I, P> Div<&'a Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Option<Monomial<I, i64, P>>;
    fn div(self, rhs: &'a Monomial<I, i64, P>) -> Self::Output {
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

impl<'a, I, P> Div<&'a Polynomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Option<Polynomial<I, i64, P>>;
    fn div(self, rhs: &'a Polynomial<I, i64, P>) -> Self::Output {
        match rhs.monomials.len() {
            1 => {
                match self / &(rhs.monomials[0]) {
                    Some(m) => Some(Polynomial { monomials: vec![m] }),
                    None => None,
                }
            }
            _ => None
        }
    }
}

impl<'a, I, P> Mul<&'a Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Monomial<I, i64, P>;
    fn mul(self, rhs: &'a Monomial<I, i64, P>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a, I, P> Mul<&'a Polynomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Polynomial<I, i64, P>;
    fn mul(self, rhs: &'a Polynomial<I, i64, P>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a, I, P> Add<&'a Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Polynomial<I, i64, P>;
    fn add(self, rhs: &'a Monomial<I, i64, P>) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a, I, P> Sub<&'a Monomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Polynomial<I, i64, P>;
    fn sub(self, rhs: &'a Monomial<I, i64, P>) -> Self::Output {
        -&(rhs.add(-self))
    }
}

impl<'a, I, P> Sub<&'a Polynomial<I, i64, P>> for i64
    where I: Id, P: Power {
    type Output = Polynomial<I, i64, P>;
    fn sub(self, rhs: &'a Polynomial<I, i64, P>) -> Self::Output {
        -&(rhs.add(-self))
    }
}
