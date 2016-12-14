use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;
use composite::Composite;

/// Returns a symbolic integer expression representing
/// the primitive variable identified by `id`.
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
        let (result, reminder) = left.div_rem(right);
        if reminder.monomials.is_empty() {
            result
        } else {
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

/// Computes a symbolic `floor` between two symbolic integer expressions.
pub fn floor<I, C, P>(left: &Polynomial<I, C, P>, right: &Polynomial<I, C, P>) -> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    if left.is_constant() && right.is_constant() {
        let v1 = left.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        let v2 = right.evaluate(&::std::collections::HashMap::default()).ok().unwrap();
        Polynomial::from(C::div_floor(&v1, &v2))
    } else {
        let (result, reminder) = left.div_rem(right);
        if reminder.monomials.is_empty() {
            result
        } else {
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
