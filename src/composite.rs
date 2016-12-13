use std::fmt;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use primitives::*;

impl<I, C> fmt::Display for Composite<I, C> where I: Id, C: Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Composite::Variable(ref id) => {
                try!(id.var_fmt(f))
            },
            &Composite::Floor(ref x, ref y) => {
                try!(write!(f, "floor({}, {})", x, y))
            },
            &Composite::Ceil(ref x, ref y) => {
                try!(write!(f, "ceil({}, {})", x, y))
            },
            &Composite::Min(ref x, ref y) => {
                try!(write!(f, "min({}, {})", x, y))
            },
            &Composite::Max(ref x, ref y) => {
                try!(write!(f, "max({}, {})", x, y))
            }
        }
        Ok(())
    }
}

impl<I, C> fmt::Debug for Composite<I, C> where I: Id, C: Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Composite::Variable(ref id) => {
                try!(id.var_fmt(f))
            },
            &Composite::Floor(ref x, ref y) => {
                try!(write!(f, "floor({:?}, {:?})", x, y))
            },
            &Composite::Ceil(ref x, ref y) => {
                try!(write!(f, "ceil({:?}, {:?})", x, y))
            },
            &Composite::Min(ref x, ref y) => {
                try!(write!(f, "min({:?}, {:?})", x, y))
            },
            &Composite::Max(ref x, ref y) => {
                try!(write!(f, "max({:?}, {:?})", x, y))
            }
        }
        Ok(())
    }
}

impl<I, C> PartialOrd for Composite<I, C> where I: Id, C: Coefficient {
    fn partial_cmp(&self, other: &Composite<I, C>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C> Ord for Composite<I, C> where I: Id, C: Coefficient {
    fn cmp(&self, other: &Composite<I, C>) -> Ordering {
        match self {
            &Composite::Variable(ref id) => {
                match other {
                    &Composite::Variable(ref o_id) => Ord::cmp(o_id, id),
                    _ => Ordering::Greater
                }
            },
            &Composite::Max(ref x, ref y) => {
                match other {
                    &Composite::Variable(_) => Ordering::Less,
                    &Composite::Max(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v
                        }
                    }
                    _ => Ordering::Greater
                }
            },
            &Composite::Min(ref x, ref y) => {
                match other {
                    &Composite::Variable(_) | &Composite::Max(_, _) => Ordering::Less,
                    &Composite::Min(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v
                        }
                    }
                    _ => Ordering::Greater
                }
            },
            &Composite::Ceil(ref x, ref y) => {
                match other {
                    &Composite::Ceil(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v
                        }
                    }
                    &Composite::Floor(_, _) => Ordering::Greater,
                    _ => Ordering::Less,
                }
            },
            &Composite::Floor(ref x, ref y) => {
                match other {
                    &Composite::Floor(ref o_x, ref o_y) => {
                        match Ord::cmp(x, o_x) {
                            Ordering::Equal => Ord::cmp(y, o_y),
                            v => v
                        }
                    }
                    _ => Ordering::Greater
                }
            }
        }
    }
}

impl<I, C> Evaluable<I, C> for Composite<I, C> where I: Id, C: Coefficient {
    fn evaluate(&self, values: &HashMap<I, C>) -> Result<C, I> {
        match self {
            &Composite::Variable(ref x) => {
                values.get(x).map(|v| v.clone()).ok_or(x.clone())
            },
            &Composite::Floor(ref x, ref y) => {
                let v_x = try!(x.evaluate(values));
                let v_y = try!(y.evaluate(values));
                Ok( C::div_floor(&v_x, &v_y) )
            } ,
            &Composite::Ceil(ref x, ref y) => {
                let v_x = try!(x.evaluate(values));
                let v_y = try!(y.evaluate(values));
                Ok( C::div_floor(&v_x, &v_y) )
            },
            &Composite::Min(ref x, ref y) => {
                let v_x = try!(x.evaluate(values));
                let v_y = try!(y.evaluate(values));
                Ok( if v_x < v_y {v_x} else {v_y} )
            },
            &Composite::Max(ref x, ref y) => {
                let v_x = try!(x.evaluate(values));
                let v_y = try!(y.evaluate(values));
                Ok( if v_x > v_y {v_x} else {v_y} )
            }
        }
    }
}