use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;
use composite::Composite;
use std::collections::HashMap;
use std::convert::AsRef;

/// Returns a polynomial representing 1 * x^1 + 0,
/// where 'x' is a variable uniquely identifiable by the provided `id`.
pub fn variable<I, C, P>(id: I) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    Polynomial {
        monomials: vec![
            Monomial {
                coefficient: C::one(),
                powers: vec![(Composite::Variable(id), P::one())],
            },
        ],
    }
}

/// Computes a symbolic `max` between two polynomials.
pub fn max<I, C, P, T1, T2>(left: T1, right: T2) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T1: AsRef<Polynomial<I, C, P>>,
          T2: AsRef<Polynomial<I, C, P>> {
    let left = left.as_ref();
    let right = right.as_ref();
    if left.is_constant() && right.is_constant() {
        let v1 = left.eval(&HashMap::default()).ok().unwrap();
        let v2 = right.eval(&HashMap::default()).ok().unwrap();
        Polynomial::from(if v1 > v2 { v1 } else { v2 })
    } else {
        Polynomial {
            monomials: vec![
                Monomial {
                    coefficient: C::one(),
                    powers: vec![
                        (
                            Composite::Max(
                                ::std::rc::Rc::new(left.clone()),
                                ::std::rc::Rc::new(right.clone()),
                            ),
                            P::one(),
                        ),
                    ],
                },
            ],
        }
    }
}

/// Computes a symbolic `min` between two polynomials.
pub fn min<I, C, P, T1, T2>(left: T1, right: T2) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T1: AsRef<Polynomial<I, C, P>>,
          T2: AsRef<Polynomial<I, C, P>> {
    let left = left.as_ref();
    let right = right.as_ref();
    if left.is_constant() && right.is_constant() {
        let v1 = left.eval(&HashMap::default()).ok().unwrap();
        let v2 = right.eval(&HashMap::default()).ok().unwrap();
        Polynomial::from(if v1 < v2 { v1 } else { v2 })
    } else {
        Polynomial {
            monomials: vec![
                Monomial {
                    coefficient: C::one(),
                    powers: vec![
                        (
                            Composite::Min(
                                ::std::rc::Rc::new(left.clone()),
                                ::std::rc::Rc::new(right.clone()),
                            ),
                            P::one(),
                        ),
                    ],
                },
            ],
        }
    }
}

/// Computes a symbolic `ceil` between two polynomials.
pub fn ceil<I, C, P, T1, T2>(left: T1, right: T2) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T1: AsRef<Polynomial<I, C, P>>,
          T2: AsRef<Polynomial<I, C, P>> {
    let left = left.as_ref();
    let right = right.as_ref();
    if left.is_constant() && right.is_constant() {
        let v1 = left.eval(&HashMap::default()).ok().unwrap();
        let v2 = right.eval(&HashMap::default()).ok().unwrap();
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
                monomials: vec![
                    Monomial {
                        coefficient: C::one(),
                        powers: vec![
                            (
                                Composite::Ceil(
                                    ::std::rc::Rc::new(left.clone()),
                                    ::std::rc::Rc::new(right.clone()),
                                ),
                                P::one(),
                            ),
                        ],
                    },
                ],
            }
        }
    }
}

/// Computes a symbolic `floor` between two polynomials.
pub fn floor<I, C, P, T1, T2>(left: T1, right: T2) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T1: AsRef<Polynomial<I, C, P>>,
          T2: AsRef<Polynomial<I, C, P>> {
    let left = left.as_ref();
    let right = right.as_ref();
    if left.is_constant() && right.is_constant() {
        let v1 = left.eval(&HashMap::default()).ok().unwrap();
        let v2 = right.eval(&HashMap::default()).ok().unwrap();
        Polynomial::from(C::div_floor(&v1, &v2))
    } else {
        let (result, reminder) = left.div_rem(right);
        if reminder.monomials.is_empty() {
            result
        } else {
            Polynomial {
                monomials: vec![
                    Monomial {
                        coefficient: C::one(),
                        powers: vec![
                            (
                                Composite::Floor(
                                    ::std::rc::Rc::new(left.clone()),
                                    ::std::rc::Rc::new(right.clone()),
                                ),
                                P::one(),
                            ),
                        ],
                    },
                ],
            }
        }
    }
}

/// Reduces the monomial, given the variable assignments provided.
pub fn reduce_monomial<I, C, P, T>(monomial: T, values: &HashMap<I, C>) -> Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T: AsRef<Monomial<I, C, P>> {
    let monomial = monomial.as_ref();
    if monomial.is_constant() {
        monomial.clone()
    } else {
        let mut result = Monomial::<I, C, P> {
            coefficient: monomial.coefficient.clone(),
            powers: Vec::new(),
        };
        for &(ref c, ref p) in &monomial.powers {
            match *c {
                Composite::Variable(ref id) => {
                    match values.get(id) {
                        Some(value) => {
                            result.coefficient *= ::num::pow(value.clone(), p.to_usize().unwrap());
                        }
                        None => {
                            result *= &Monomial::<I, C, P> {
                                coefficient: C::one(),
                                powers: vec![(c.clone(), p.clone())],
                            };
                        }
                    }
                }
                Composite::Max(ref left, ref right) => {
                    let mut reduced_left = ::std::rc::Rc::new(reduce(&*left, &*values));
                    let mut reduced_right = ::std::rc::Rc::new(reduce(&*right, &*values));
                    if reduced_left.eq(left) {
                        reduced_left = left.clone();
                    }
                    if reduced_right.eq(right) {
                        reduced_right = right.clone();
                    }
                    let c = Composite::Max(reduced_left.clone(), reduced_right.clone());
                    if reduced_left.is_constant() && reduced_right.is_constant() {
                        result.coefficient *= c.eval(values).unwrap();
                    } else {
                        result *= &Monomial::<I, C, P> {
                            coefficient: C::one(),
                            powers: vec![(c, p.clone())],
                        };
                    }
                }
                Composite::Min(ref left, ref right) => {
                    let mut reduced_left = ::std::rc::Rc::new(reduce(&*left, &*values));
                    let mut reduced_right = ::std::rc::Rc::new(reduce(&*right, &*values));
                    if reduced_left.eq(left) {
                        reduced_left = left.clone();
                    }
                    if reduced_right.eq(right) {
                        reduced_right = right.clone();
                    }
                    let c = Composite::Min(reduced_left.clone(), reduced_right.clone());
                    if reduced_left.is_constant() && reduced_right.is_constant() {
                        result.coefficient *= c.eval(values).unwrap();
                    } else {
                        result *= &Monomial::<I, C, P> {
                            coefficient: C::one(),
                            powers: vec![(c, p.clone())],
                        };
                    }
                }
                Composite::Ceil(ref left, ref right) => {
                    let mut reduced_left = ::std::rc::Rc::new(reduce(&*left, &*values));
                    let mut reduced_right = ::std::rc::Rc::new(reduce(&*right, &*values));
                    if reduced_left.eq(left) {
                        reduced_left = left.clone();
                    }
                    if reduced_right.eq(right) {
                        reduced_right = right.clone();
                    }
                    let c = Composite::Ceil(reduced_left.clone(), reduced_right.clone());
                    if reduced_left.is_constant() && reduced_right.is_constant() {
                        result.coefficient *= c.eval(values).unwrap();
                    } else {
                        result *= &Monomial::<I, C, P> {
                            coefficient: C::one(),
                            powers: vec![(c, p.clone())],
                        };
                    }
                }
                Composite::Floor(ref left, ref right) => {
                    let mut reduced_left = ::std::rc::Rc::new(reduce(&*left, &*values));
                    let mut reduced_right = ::std::rc::Rc::new(reduce(&*right, &*values));
                    if reduced_left.eq(left) {
                        reduced_left = left.clone();
                    }
                    if reduced_right.eq(right) {
                        reduced_right = right.clone();
                    }
                    let c = Composite::Floor(reduced_left.clone(), reduced_right.clone());
                    if reduced_left.is_constant() && reduced_right.is_constant() {
                        result.coefficient *= c.eval(values).unwrap();
                    } else {
                        result *= &Monomial::<I, C, P> {
                            coefficient: C::one(),
                            powers: vec![(c, p.clone())],
                        };
                    }
                }
            }
        }
        result
    }
}

/// Reduces the polynomial, given the variable assignments provided.
pub fn reduce<I, C, P, T>(polynomial: T, values: &HashMap<I, C>) -> Polynomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power,
          T: AsRef<Polynomial<I, C, P>> {
    let polynomial = polynomial.as_ref();
    let mut result = Polynomial::<I, C, P> { monomials: Vec::new() };
    for m in &polynomial.monomials {
        result += &reduce_monomial(m, values);
    }
    result
}

/// Automatically deduces the individual variable assignments based on the
/// system of equations specified by the mapping of `Polynomial` to a constant value.
pub fn deduce_values<I, C, P, T>(original_values: &[(T, C)]) -> Result<HashMap<I, C>, String>
    where I: Id,
          C: Coefficient,
          P: Power,
          T: AsRef<Polynomial<I, C, P>> {
    //    let mut implicit_values = vec![(Polynomial::default(), C::zero()); original_values.len()];
    let mut implicit_values = original_values
        .iter()
        .map(|&(ref p, ref c)| (p.as_ref().clone(), c.clone()))
        .collect::<Vec<(Polynomial<I, C, P>, C)>>();
    let mut verified = vec![false; original_values.len()];
    //    let mut indexes: Vec<usize> = (0..original_values.len()).collect();
    let mut values: HashMap<I, C> = HashMap::new();
    let mut i = 0;
    while i < implicit_values.len() {
        if verified[i] {
            i += 1;
            continue;
        }
        let to_reduce = {
            let (ref p, ref c) = implicit_values[i];
            if p.is_constant() {
                let value = p.eval(&HashMap::new()).unwrap();
                if value != *c {
                    return Err(format!(
                        "Value deduction failed for {} = {}, as it was deduced to {}.",
                        original_values[i].0.as_ref(),
                        c,
                        value
                    ));
                } else {
                    verified[i] = true;
                    false
                }
            } else if (p.monomials.len() == 1 ||
                       (p.monomials.len() == 2 && p.monomials[1].is_constant())) &&
                      p.monomials[0].powers.len() == 1 {
                if let Composite::Variable(ref id) = p.monomials[0].powers[0].0 {
                    // The polynomial is in the form a * x^n + b, so we can deduce value of 'x'
                    let b = p.monomials
                        .get(1)
                        .map_or(C::zero(), |m| m.eval(&values).unwrap());
                    let a = &p.monomials[0].coefficient;
                    let n = &p.monomials[0].powers[0].1;
                    let inferred =
                        match nth_root(&((c.clone() - b.clone()) / a.clone()), n.clone()) {
                            Some(val) => val,
                            None => {
                                return Err(format!(
                                    "Could not find integer solution to {} * {}^{} + {} = {}.",
                                    a,
                                    id,
                                    n,
                                    b,
                                    c
                                ))
                            }
                        };
                    values.insert(id.clone(), inferred);
                    verified[i] = true;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        //        if to_remove {
        //            implicit_values.remove(i);
        //            indexes.remove(i);
        //            if implicit_values.is_empty() {
        //                return Ok(values);
        //            }
        //        }
        if to_reduce {
            for &mut (ref mut p, _) in &mut implicit_values {
                *p = reduce(&p, &values);
            }
            i = 0;
        } else {
            i += 1;
        }
    }
    if !verified.iter().fold(true, |all, &x| all && x) {
        Err("Could not deduce all variables.".into())
    } else {
        Ok(values)
    }

}

fn nth_root<C, P>(value: &C, n: P) -> Option<C>
    where C: Coefficient,
          P: Power {
    let result = if value < &C::zero() {
        C::from_f64(-(-value.to_f64().unwrap()).powf(n.to_f64().unwrap().recip())).unwrap()
    } else {
        C::from_f64(value.to_f64().unwrap().powf(n.to_f64().unwrap().recip())).unwrap()
    };
    if ::num::pow(result.clone(), n.to_usize().unwrap()) == *value {
        Some(result)
    } else {
        None
    }
}
