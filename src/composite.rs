use std::cmp::{Ord, Ordering};
use traits::*;
use polynomial::Polynomial;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// A composite expression (tagged union) of a variable or an irreducible function (floor, ceil, max, min)
pub enum Composite<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    Variable(I),
    Floor(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Ceil(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Min(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Max(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
}

impl<I, C, P> Composite<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    /// Evaluates the `Composite` given the provided mapping of identifier to value assignment.
    pub fn eval(&self, values: &::std::collections::HashMap<I, C>) -> Result<C, (I, String)> {
        match *self {
            Composite::Variable(ref x) => {
                values.get(x)
                    .cloned()
                    .ok_or((x.clone(), format!("Value not provided for {}.", x)))
            }
            Composite::Floor(ref x, ref y) => {
                let v_x = x.eval(values)?;
                let v_y = y.eval(values)?;
                if v_y == C::zero() {
                    Err((I::default(), "Attempting division by zero.".to_string()))
                } else {
                    Ok(C::div_floor(&v_x, &v_y))
                }
            }
            Composite::Ceil(ref x, ref y) => {
                let v_x = x.eval(values)?;
                let v_y = y.eval(values)?;
                if v_y == C::zero() {
                    Err((I::default(), "Attempting division by zero.".to_string()))
                } else {
                    let (d, rem) = v_x.div_rem(&v_y);
                    if rem == C::zero() {
                        Ok(d)
                    } else {
                        Ok(d + C::one())
                    }
                }
            }
            Composite::Min(ref x, ref y) => {
                let v_x = x.eval(values)?;
                let v_y = y.eval(values)?;
                Ok(if v_x < v_y { v_x } else { v_y })
            }
            Composite::Max(ref x, ref y) => {
                let v_x = x.eval(values)?;
                let v_y = y.eval(values)?;
                Ok(if v_x > v_y { v_x } else { v_y })
            }
        }
    }

    /// Returns a code equivalent string representation of the `Composite`.
    /// The `format` specifies a function how to render the identifiers;
    pub fn to_code<F>(&self, format: &F) -> String
        where F: ::std::ops::Fn(I) -> String {
        let mut str: String = "".into();
        match *self {
            Composite::Variable(ref id) => str = format(id.clone()),
            Composite::Floor(_, _) => str.push_str("floor("),
            Composite::Ceil(_, _) => str.push_str("ceil("),
            Composite::Max(_, _) => str.push_str("max("),
            Composite::Min(_, _) => str.push_str("min("),
        }
        match *self {
            Composite::Variable(_) => {}
            Composite::Floor(ref x, ref y) |
            Composite::Ceil(ref x, ref y) |
            Composite::Max(ref x, ref y) |
            Composite::Min(ref x, ref y) => {
                str.push_str(&x.to_code(format));
                str.push_str(", ");
                str.push_str(&y.to_code(format));
                str.push_str(")");
            }
        }
        str
    }
}


impl<I, C, P> ::std::fmt::Display for Composite<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Composite::Variable(ref id) => write!(f, "{}", id),
            Composite::Floor(ref x, ref y) => write!(f, "floor({}, {})", x, y),
            Composite::Ceil(ref x, ref y) => write!(f, "ceil({}, {})", x, y),
            Composite::Min(ref x, ref y) => write!(f, "min({}, {})", x, y),
            Composite::Max(ref x, ref y) => write!(f, "max({}, {})", x, y),
        }
    }
}



impl<I, C, P> PartialOrd for Composite<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &Composite<I, C, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C, P> Ord for Composite<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn cmp(&self, other: &Composite<I, C, P>) -> Ordering {
        match *self {
            Composite::Variable(ref id) => {
                match *other {
                    Composite::Variable(ref o_id) => Ord::cmp(o_id, id),
                    _ => Ordering::Greater
                }
            }
            Composite::Max(ref x, ref y) => {
                match *other {
                    Composite::Variable(_) => Ordering::Less,
                    Composite::Max(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v,
                        }
                    }
                    _ => Ordering::Greater
                }
            }
            Composite::Min(ref x, ref y) => {
                match *other {
                    Composite::Variable(_) |
                    Composite::Max(_, _) => Ordering::Less,
                    Composite::Min(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v,
                        }
                    }
                    _ => Ordering::Greater
                }
            }
            Composite::Ceil(ref x, ref y) => {
                match *other {
                    Composite::Ceil(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v,
                        }
                    }
                    Composite::Floor(_, _) => Ordering::Greater,
                    _ => Ordering::Less
                }
            }
            Composite::Floor(ref x, ref y) => {
                match *other {
                    Composite::Floor(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v,
                        }
                    }
                    _ => Ordering::Greater
                }
            }
        }
    }
}
