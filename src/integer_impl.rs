use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;

macro_rules! impl_all {
    ( $( $type_:ty ),* )  => {$(
        impl<I, C, P> PartialEq<Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            fn eq( & self, other: & Monomial < I, C, P > ) -> bool {
                other.eq( & C::from( * self ))
            }
        }
        impl<I, C, P> PartialEq<Polynomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            fn eq(&self, other: &Polynomial<I, C, P>) -> bool {
                other.eq(&C::from(*self))
            }
        }
        impl<I, C, P> PartialOrd<Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            fn partial_cmp(&self, other: &Monomial<I, C, P>) -> Option<Ordering> {
                match other.partial_cmp(&C::from(*self)) {
                    Some(Ordering::Less) => Some(Ordering::Greater),
                    Some(Ordering::Equal) => Some(Ordering::Equal),
                    Some(Ordering::Greater) => Some(Ordering::Less),
                    None => None,
                }
            }
        }
        impl<I, C, P> PartialOrd<Polynomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            fn partial_cmp(&self, other: &Polynomial<I, C, P>) -> Option<Ordering> {
                match other.partial_cmp(&C::from(*self)) {
                    Some(Ordering::Less) => Some(Ordering::Greater),
                    Some(Ordering::Equal) => Some(Ordering::Equal),
                    Some(Ordering::Greater) => Some(Ordering::Less),
                    None => None,
                }
            }
        }
        impl<'a, I, C, P> Div<&'a Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Option<Monomial<I, C, P>>;
            fn div(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
                if rhs.is_constant() {
                    let c = C::from(self);
                    match c.checked_div(&rhs.coefficient) {
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
        impl<I, C, P> Div<Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Option<Monomial<I, C, P>>;
            fn div(self, rhs: Monomial<I, C, P>) -> Self::Output {
                self.div(&rhs)
            }
        }
        impl<'a, I, C, P> Div<&'a Polynomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Option<Polynomial<I, C, P>>;
            fn div(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
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
        impl<I, C, P> Div<Polynomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Option<Polynomial<I, C, P>>;
            fn div(self, rhs: Polynomial<I, C, P>) -> Self::Output {
                self.div(&rhs)
            }
        }
        impl<'a, I, C, P> Mul<&'a Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Monomial<I, C, P>;
            fn mul(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
                rhs.mul(C::from(self))
            }
        }
        impl<I, C, P> Mul<Monomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Monomial<I, C, P>;
            fn mul(self, rhs: Monomial<I, C, P>) -> Self::Output {
                (&rhs).mul(C::from(self))
            }
        }
        impl<'a, I, C, P> Mul<&'a Polynomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn mul(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
                rhs.mul(C::from(self))
            }
        }
        impl<I, C, P> Mul<Polynomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn mul(self, rhs: Polynomial<I, C, P>) -> Self::Output {
                (&rhs).mul(C::from(self))
            }
        }
        impl<'a, I, C, P> Add<&'a Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn add(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
                rhs.add(C::from(self))
            }
        }
        impl<I, C, P> Add<Monomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn add(self, rhs: Monomial<I, C, P>) -> Self::Output {
                (&rhs).add(C::from(self))
            }
        }
        impl<'a, I, C, P> Add<&'a Polynomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn add(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
                rhs.add(C::from(self))
            }
        }
        impl<I, C, P> Add<Polynomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn add(self, rhs: Polynomial<I, C, P>) -> Self::Output {
                (&rhs).add(C::from(self))
            }
        }
        impl<'a, I, C, P> Sub<&'a Monomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn sub(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
                -&(rhs.add(-C::from(self)))
            }
        }

        impl<I, C, P> Sub<Monomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn sub(self, rhs: Monomial<I, C, P>) -> Self::Output {
                -&((&rhs).add(-C::from(self)))
            }
        }

        impl<'a, I, C, P> Sub<&'a Polynomial<I, C, P>> for $type_
            where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn sub(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
                -&(rhs.add(-C::from(self)))
            }
        }

        impl<I, C, P> Sub<Polynomial<I, C, P>> for $type_
        where I: Id, P: Power, C: Coefficient + From<$type_> {
            type Output = Polynomial<I, C, P>;
            fn sub(self, rhs: Polynomial<I, C, P>) -> Self::Output {
                -&((&rhs).add(-C::from(self)))
            }
        }
    )*};
}

// impl_all!(i64, u64, i32, u32, i16, u16, i8, u8, usize);
impl_all!(i64);
