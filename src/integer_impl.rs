use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;

macro_rules! impl_all {
    ( $( $type_:ty ),* )  => {$(
        impl<I, P> PartialEq<Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            fn eq( & self, other: & Monomial < I, $type_, P > ) -> bool {
                other.eq(self)
            }
        }
        impl<I, P> PartialEq<Polynomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            fn eq(&self, other: &Polynomial<I, $type_, P>) -> bool {
                other.eq(self)
            }
        }
        impl<I, P> PartialOrd<Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            fn partial_cmp(&self, other: &Monomial<I, $type_, P>) -> Option<Ordering> {
                match other.partial_cmp(self) {
                    Some(Ordering::Less) => Some(Ordering::Greater),
                    Some(Ordering::Equal) => Some(Ordering::Equal),
                    Some(Ordering::Greater) => Some(Ordering::Less),
                    None => None,
                }
            }
        }
        impl<I, P> PartialOrd<Polynomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            fn partial_cmp(&self, other: &Polynomial<I, $type_, P>) -> Option<Ordering> {
                match other.partial_cmp(self) {
                    Some(Ordering::Less) => Some(Ordering::Greater),
                    Some(Ordering::Equal) => Some(Ordering::Equal),
                    Some(Ordering::Greater) => Some(Ordering::Less),
                    None => None,
                }
            }
        }
        impl<'a, I, P> Div<&'a Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Option<Monomial<I, $type_, P>>;
            fn div(self, rhs: &'a Monomial<I, $type_, P>) -> Self::Output {
                if rhs.is_constant() {
                    let c = self;
                    match c.checked_div(rhs.coefficient) {
                        Some(v) => {
                            if v * c.clone() == rhs.coefficient {
                                Some(Monomial::from(c / rhs.coefficient.clone()))
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            }
        }
        impl<I, P> Div<Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Option<Monomial<I, $type_, P>>;
            fn div(self, rhs: Monomial<I, $type_, P>) -> Self::Output {
                self.div(&rhs)
            }
        }
        impl<'a, I, P> Div<&'a Polynomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Option<Polynomial<I, $type_, P>>;
            fn div(self, rhs: &'a Polynomial<I, $type_, P>) -> Self::Output {
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
        impl<I, P> Div<Polynomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Option<Polynomial<I, $type_, P>>;
            fn div(self, rhs: Polynomial<I, $type_, P>) -> Self::Output {
                self.div(&rhs)
            }
        }
        impl<'a, I, P> Mul<&'a Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Monomial<I, $type_, P>;
            fn mul(self, rhs: &'a Monomial<I, $type_, P>) -> Self::Output {
                rhs.mul(self)
            }
        }
        impl<I, P> Mul<Monomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Monomial<I, $type_, P>;
            fn mul(self, rhs: Monomial<I, $type_, P>) -> Self::Output {
                (&rhs).mul(self)
            }
        }
        impl<'a, I, P> Mul<&'a Polynomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn mul(self, rhs: &'a Polynomial<I, $type_, P>) -> Self::Output {
                rhs.mul(self)
            }
        }
        impl<I, P> Mul<Polynomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn mul(self, rhs: Polynomial<I, $type_, P>) -> Self::Output {
                (&rhs).mul(self)
            }
        }
        impl<'a, I, P> Add<&'a Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn add(self, rhs: &'a Monomial<I, $type_, P>) -> Self::Output {
                rhs.add(self)
            }
        }
        impl<I, P> Add<Monomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn add(self, rhs: Monomial<I, $type_, P>) -> Self::Output {
                (&rhs).add(self)
            }
        }
        impl<'a, I, P> Add<&'a Polynomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn add(self, rhs: &'a Polynomial<I, $type_, P>) -> Self::Output {
                rhs.add(self)
            }
        }
        impl<I, P> Add<Polynomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn add(self, rhs: Polynomial<I, $type_, P>) -> Self::Output {
                (&rhs).add(self)
            }
        }
        impl<'a, I, P> Sub<&'a Monomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn sub(self, rhs: &'a Monomial<I, $type_, P>) -> Self::Output {
                -&(rhs.add(-self))
            }
        }

        impl<I, P> Sub<Monomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn sub(self, rhs: Monomial<I, $type_, P>) -> Self::Output {
                -&((&rhs).add(-self))
            }
        }

        impl<'a, I, P> Sub<&'a Polynomial<I, $type_, P>> for $type_
            where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn sub(self, rhs: &'a Polynomial<I, $type_, P>) -> Self::Output {
                -&(rhs.add(-self))
            }
        }

        impl<I, P> Sub<Polynomial<I, $type_, P>> for $type_
        where I: Id, P: Power {
            type Output = Polynomial<I, $type_, P>;
            fn sub(self, rhs: Polynomial<I, $type_, P>) -> Self::Output {
                -&((&rhs).add(-self))
            }
        }
    )*};
}

impl_all!(i64, i32, i16, i8, isize);
