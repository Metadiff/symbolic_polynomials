use std::fmt;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use primitives::*;

impl fmt::Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Composite::Variable(id) => {
                try!(write!(f, "{}", (id as u8 + ('a' as u8)) as char))
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

impl fmt::Debug for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Composite::Variable(id) => {
                try!(write!(f, "{}", (id as u8 + ('a' as u8)) as char))
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

impl PartialOrd for Composite {
    fn partial_cmp(&self, other: &Composite) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Composite {
    fn cmp(&self, other: &Composite) -> Ordering {
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

impl Evaluable for Composite {
    fn evaluate(&self, values: &HashMap<u16, i64>) -> Result<i64, u16> {
        match self {
            &Composite::Variable(ref x) => {
                values.get(x).map(|&v| v).ok_or(*x)
            },
            &Composite::Floor(ref x, ref y) => {
                let v_x = try!(x.evaluate(values)) as f64;
                let v_y = try!(y.evaluate(values)) as f64;
                Ok( (v_x / v_y).floor() as i64)
            } ,
            &Composite::Ceil(ref x, ref y) => {
                let v_x = try!(x.evaluate(values)) as f64;
                let v_y = try!(y.evaluate(values)) as f64;
                Ok( (v_x / v_y).ceil() as i64)
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