use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::cmp::{Ord, Ordering};

use monomial::Monomial;
use functions::*;

#[derive(Clone, Default, Debug, Eq)]
pub struct Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    pub monomials: Vec<Monomial<I, C, P>>,
}

impl<I, C, P> IsConstant for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn is_constant(&self) -> bool {
        match self.monomials.len() {
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }
}

impl<I, C, P> Evaluable<I, C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn evaluate(&self, values: &::std::collections::HashMap<I, C>) -> Result<C, I> {
        let mut value = C::zero();
        for m in &self.monomials {
            value += m.evaluate(values)?;
        }
        Ok(value)
    }
}

impl<I, C, P> ::std::fmt::Display for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self.monomials.len() {
            0 => write!(f, "0"),
            _ => {
                write!(f, "{}", self.monomials[0])?;
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > C::zero() {
                        write!(f, " + {}", m)?;
                    } else {
                        write!(f, " {}", m)?;
                    }
                }
                Ok(())
            }
        }
    }
}

// impl<I, C, P> ::std::fmt::Debug for Polynomial<I, C, P> where I: Id, C: Coefficient, P: Power {
//    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
//        match self.monomials.len() {
//            0 => write!(f, "0"),
//            _ => {
//                write!(f, "{:?}", self.monomials[0])?;
//                for m in self.monomials.iter().skip(1) {
//                    if m.coefficient > C::zero() {
//                        write!(f, " + {:?}", m)?;
//                    } else {
//                        write!(f, " {:?}", m)?;
//                    }
//                }
//                Ok(())
//            }
//        }
//    }
//

impl<I, C, P> From<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn from(other: C) -> Self {
        Polynomial { monomials: vec![Monomial::from(other)] }
    }
}

impl<'a, I, C, P> From<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn from(m: &'a Monomial<I, C, P>) -> Self {
        Polynomial { monomials: vec![m.clone()] }
    }
}

impl<I, C, P> From<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn from(m: Monomial<I, C, P>) -> Self {
        Polynomial { monomials: vec![m] }
    }
}

impl<I, C, P> PartialEq<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn eq(&self, other: &C) -> bool {
        match self.monomials.len() {
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I, C, P> PartialEq<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn eq(&self, other: &Monomial<I, C, P>) -> bool {
        match self.monomials.len() {
            0 => other.coefficient == C::zero(),
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I, C, P> PartialEq for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn eq(&self, other: &Polynomial<I, C, P>) -> bool {
        if self.monomials.len() == other.monomials.len() {
            for (ms, mo) in self.monomials.iter().zip(other.monomials.iter()) {
                if ms != mo {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl<I, C, P> PartialOrd<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        match self.monomials.len() {
            0 => {
                match other.partial_cmp(&C::zero()) {
                    Some(Ordering::Greater) => Some(Ordering::Less),
                    Some(Ordering::Equal) => Some(Ordering::Equal),
                    Some(Ordering::Less) => Some(Ordering::Greater),
                    None => None,
                }
            }
            1 => self.monomials[0].partial_cmp(other),
            _ => Some(Ordering::Greater)
        }
    }
}

impl<I, C, P> PartialOrd<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &Monomial<I, C, P>) -> Option<Ordering> {
        match self.monomials.len() {
            0 => {
                if other.is_constant() {
                    C::zero().partial_cmp(&other.coefficient)
                } else {
                    Some(Ordering::Less)
                }
            }
            1 => self.monomials[0].partial_cmp(other),
            _ => Some(Ordering::Greater)
        }
    }
}

impl<I, C, P> PartialOrd for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &Polynomial<I, C, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C, P> Ord for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn cmp(&self, other: &Polynomial<I, C, P>) -> Ordering {
        let m = if self.monomials.len() < other.monomials.len() {
            self.monomials.len()
        } else {
            other.monomials.len()
        };
        for i in 0..m {
            match Ord::cmp(&self.monomials[i], &other.monomials[i]) {
                Ordering::Equal => {}
                result => return result,
            }
        }
        Ord::cmp(&self.monomials.len(), &other.monomials.len())
    }
}

impl<I, C, P> MulAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: C) {
        for m in &mut self.monomials {
            *m *= rhs.clone();
        }
    }
}

impl<'a, I, C, P> Mul<C> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C, P> MulAssign<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        for m in &mut self.monomials {
            *m *= rhs;
        }
    }
}

impl<'a, 'b, I, C, P> Mul<&'a Monomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C, P> MulAssign<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: &'a Polynomial<I, C, P>) {
        if !self.monomials.is_empty() {
            let mut result = Polynomial { monomials: Vec::new() };
            for m in &self.monomials {
                result += &(m * rhs);
            }
            self.monomials = result.monomials;
        }
    }
}

impl<'a, 'b, I, C, P> Mul<&'a Polynomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<I, C, P> CheckedDiv<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn checked_div(&self, rhs: C) -> Option<Self::Output> {
        let result = Polynomial {
            monomials: self.monomials
                .iter()
                .cloned()
                .filter_map(|ref m| m.checked_div(rhs.clone()))
                .collect(),
        };
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, I, C, P> Div<C> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: C) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<I, C, P> DivAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: C) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

impl<'a, I, C, P> CheckedDiv<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn checked_div(&self, rhs: &'a Monomial<I, C, P>) -> Option<Self::Output> {
        let result = Polynomial {
            monomials: self.monomials
                .iter()
                .cloned()
                .filter_map(|ref m| m.checked_div(rhs))
                .collect(),
        };
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, 'b, I, C, P> Div<&'a Monomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C, P> DivAssign<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

impl<'a, I, C, P> CheckedDiv<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn checked_div(&self, rhs: &'a Polynomial<I, C, P>) -> Option<Self::Output> {
        let mut result = Polynomial { monomials: Vec::new() };
        let mut reminder = self.clone();
        while !reminder.is_constant() {
            match (reminder.monomials[0]).checked_div(&rhs.monomials[0]) {
                Some(ref x) => {
                    result += x;
                    reminder -= &(rhs * x);
                }
                None => return None,
            }
        }
        if !reminder.monomials.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, 'b, I, C, P> Div<&'a Polynomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C, P> DivAssign<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: &'a Polynomial<I, C, P>) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

impl<'a, I, C, P> Neg for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn neg(self) -> Self::Output {
        Polynomial { monomials: self.monomials.iter().cloned().map(|ref x| -x).collect() }
    }
}

impl<I, C, P> AddAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: C) {
        if rhs != C::zero() {
            let mut remove: bool = false;
            if let Some(ref mut l) = self.monomials.last_mut() {
                if l.is_constant() {
                    l.coefficient += rhs.clone();
                    if l.coefficient == C::zero() {
                        remove = true;
                    }
                }
            }
            if remove {
                self.monomials.pop();
            } else {
                self.monomials.push(Monomial::from(rhs));
            }
        }
    }
}

impl<'a, I, C, P> Add<C> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I, C, P> AddAssign<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        if rhs.coefficient != C::zero() {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient += rhs.coefficient.clone();
                    if self.monomials[i].coefficient == C::zero() {
                        self.monomials.remove(i);
                    }
                    return;
                }
                if let Ordering::Less = Ord::cmp(&(self.monomials[i]), rhs) {
                    self.monomials.insert(i, rhs.clone());
                    return;
                }
            }
            self.monomials.push(rhs.clone());
        }
    }
}

impl<'a, 'b, I, C, P> Add<&'a Monomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I, C, P> AddAssign<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: &'a Polynomial<I, C, P>) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]) {
                self.monomials[i1].coefficient += rhs.monomials[i2].coefficient.clone();
                if self.monomials[i1].coefficient == C::zero() {
                    self.monomials.remove(i1);
                } else {
                    i1 += 1;
                }
                i2 += 1;
            } else if self.monomials[i1] > rhs.monomials[i2] {
                i1 += 1;
            } else {
                self.monomials.insert(i1, rhs.monomials[i2].clone());
                i1 += 1;
                i2 += 2;
            }
        }
        while i2 < rhs.monomials.len() {
            self.monomials.push(rhs.monomials[i2].clone());
            i2 += 1;
        }
    }
}

impl<'a, 'b, I, C, P> Add<&'a Polynomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<I, C, P> SubAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn sub_assign(&mut self, rhs: C) {
        if rhs != C::zero() {
            let mut remove: bool = false;
            if let Some(ref mut l) = self.monomials.last_mut() {
                if l.is_constant() {
                    if rhs == l.coefficient {
                        remove = true;
                    } else {
                        l.coefficient -= rhs;
                        return;
                    }
                }
            }
            if remove {
                self.monomials.pop();
            } else {
                self.monomials.push(Monomial::from(-rhs));
            }
        }
    }
}

impl<'a, I, C, P> Sub<C> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I, C, P> SubAssign<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn sub_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        if rhs.coefficient != C::zero() {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient -= rhs.coefficient.clone();
                    if self.monomials[i].coefficient == C::zero() {
                        self.monomials.remove(i);
                    }
                    return;
                }
                if let Ordering::Less = Ord::cmp(&(self.monomials[i]), rhs) {
                    self.monomials.insert(i, rhs.clone());
                    return;
                }
            }
            self.monomials.push(rhs.clone());
        }
    }
}

impl<'a, 'b, I, C, P> Sub<&'a Monomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I, C, P> SubAssign<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn sub_assign(&mut self, rhs: &'a Polynomial<I, C, P>) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]) {
                self.monomials[i1].coefficient -= rhs.monomials[i2].coefficient.clone();
                if self.monomials[i1].coefficient == C::zero() {
                    self.monomials.remove(i1);
                } else {
                    i1 += 1;
                }
                i2 += 1;
            } else if self.monomials[i1] > rhs.monomials[i2] {
                i1 += 1;
            } else {
                self.monomials.insert(i1, -&(rhs.monomials[i2]));
                i1 += 1;
                i2 += 2;
            }
        }
        while i2 < rhs.monomials.len() {
            self.monomials.push(-&(rhs.monomials[i2]));
            i2 += 1;
        }
    }
}

impl<'a, 'b, I, C, P> Sub<&'a Polynomial<I, C, P>> for &'b Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}
