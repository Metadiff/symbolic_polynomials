use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{Ordering};
use std::convert::From;

use primitives::*;

impl PartialEq<Monomial> for i64 {
    fn eq(&self, other: &Monomial) -> bool {
        other.eq(self)
    }
}

impl PartialEq<Polynomial> for i64 {
    fn eq(&self, other: &Polynomial) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Monomial> for i64 {
    fn partial_cmp(&self, other: &Monomial) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl PartialOrd<Polynomial> for i64 {
    fn partial_cmp(&self, other: &Polynomial) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<'a> Div<&'a Monomial> for i64{
    type Output = Option<Monomial>;
    fn div(self, rhs: &'a Monomial) -> Self::Output {
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

impl<'a> Div<&'a Polynomial> for i64{
    type Output = Option<Polynomial>;
    fn div(self, rhs: &'a Polynomial) -> Self::Output {
        match rhs.monomials.len() {
            1 => match self / &(rhs.monomials[0]) {
                Some(m) => Some(Polynomial{monomials: vec![m]}),
                None => None
            },
            _ => None
        }
    }
}

impl<'a> Mul<&'a Monomial> for i64{
    type Output = Monomial;
    fn mul(self, rhs: &'a Monomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Mul<&'a Polynomial> for i64{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Polynomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Add<&'a Monomial> for i64{
    type Output = Polynomial;
    fn add(self, rhs: &'a Monomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Add<&'a Polynomial> for i32{
    type Output = Polynomial;
    fn add(self, rhs: &'a Polynomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Sub<&'a Monomial> for i64{
    type Output = Polynomial;
    fn sub(self, rhs: &'a Monomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}

impl<'a> Sub<&'a Polynomial> for i64{
    type Output = Polynomial;
    fn sub(self, rhs: &'a Polynomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}
