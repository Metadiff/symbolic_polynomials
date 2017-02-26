use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::cmp::{Ord, Ordering};
use std::convert::AsRef;
use std::collections::HashSet;

use traits::*;
use monomial::Monomial;

#[derive(Clone, Default, Debug, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr_c", repr(C))]
/// A symbolic polynomial represented as  `m_1` + `m_2` + ... + `m_n`.
pub struct Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    /// A vector of the monomials m_i, where m_i is a Monomial.
    pub monomials: Vec<Monomial<I, C, P>>,
}

impl<I, C, P> AsRef<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<I, C, P> Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    /// `True` only if the polynomial is constant and does not depend on any symbolic variables.
    pub fn is_constant(&self) -> bool {
        match self.monomials.len() {
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }

    /// Evaluates the `Polynomial` given the provided mapping of identifiers to value assignments.
    pub fn eval(&self, values: &::std::collections::HashMap<I, C>) -> Result<C, (I, String)> {
        let mut value = C::zero();
        for m in &self.monomials {
            value += m.eval(values)?;
        }
        Ok(value)
    }

    /// Returns a code equivalent string representation of the `Polynomial`.
    /// The `format` specifies a function how to render the identifiers.
    pub fn to_code<F>(&self, format: &F) -> String
        where F: ::std::ops::Fn(I) -> String {
        match self.monomials.len() {
            0 => "0".into(),
            _ => {
                let mut str: String;
                str = self.monomials[0].to_code(format);
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > C::zero() {
                        str += " + ";
                        str += &m.to_code(format);
                    } else {
                        str += " ";
                        str += &m.to_code(format);
                    }
                }
                str
            }
        }
    }

    /// Returns the result of the polynomial division with `rhs` as well as the reminder.
    /// Note that this division depends on the ordering of the variable variables type `I`
    /// as explained in [Wikipedia](https://en.wikipedia.org/wiki/Gr%C3%B6bner_basis#Reduction).
    pub fn div_rem(&self, rhs: &Polynomial<I, C, P>) -> (Polynomial<I, C, P>, Polynomial<I, C, P>) {
        let mut result = Polynomial { monomials: Vec::new() };
        let mut reminder = self.clone();
        while !reminder.monomials.is_empty() {
            match (reminder.monomials[0]).checked_div(&rhs.monomials[0]) {
                Some(ref x) => {
                    result += x;
                    reminder -= &(rhs * x);
                }
                None => return (result, reminder),
            }
        }
        (result, reminder)
    }

    /// If the the polynomial is divisible by `rhs` than returns the result
    /// of that division, otherwise None.
    pub fn checked_div(&self, rhs: &Polynomial<I, C, P>) -> Option<Polynomial<I, C, P>> {
        let (result, reminder) = self.div_rem(rhs);
        if reminder.monomials.is_empty() {
            Some(result)
        } else {
            None
        }
    }

    /// Fills into the `HashSet` all of the identifiers used in this `Polynomial`.
    pub fn unique_identifiers(&self, unique: &mut HashSet<I>) {
        for m in &self.monomials {
            m.unique_identifiers(unique);
        }
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
        let m = ::std::cmp::min(self.monomials.len(), other.monomials.len());
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

impl<I, C, P> Mul<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: C) -> Self::Output {
        (&self).mul(rhs)
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

impl<'a, 'b, I, C, P> Mul<&'b Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'b, I, C, P> Mul<&'b Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, I, C, P> Mul<Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<I, C, P> Mul<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).mul(&rhs)
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

impl<I, C, P> MulAssign<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: Polynomial<I, C, P>) {
        self.mul_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Mul<&'b Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'b, I, C, P> Mul<&'b Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, I, C, P> Mul<Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<I, C, P> Mul<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).mul(&rhs)
    }
}

// impl<I, C, P> CheckedDiv<C> for Polynomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//    type Output = Polynomial<I, C, P>;
//    fn checked_div(&self, rhs: C) -> Option<Self::Output> {
//        let result = Polynomial {
//            monomials: self.monomials
//                .iter()
//                .cloned()
//                .filter_map(|ref m| m.checked_div(rhs.clone()))
//                .collect(),
//        };
//        if result.monomials.len() != self.monomials.len() {
//            None
//        } else {
//            Some(result)
//        }
//    }
//

impl<'a, I, C, P> Div<C> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: C) -> Self::Output {
        match self.checked_div(&rhs.clone().into()) {
            Some(result) => result,
            None => panic!("Trying to divide {} by {} which is not possible.", self, rhs),
        }
    }
}

impl<I, C, P> Div<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: C) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<I, C, P> DivAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: C) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

// impl<'a, I, C, P> CheckedDiv<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//    type Output = Polynomial<I, C, P>;
//    fn checked_div(&self, rhs: &'a Monomial<I, C, P>) -> Option<Self::Output> {
//        let result = Polynomial {
//            monomials: self.monomials
//                .iter()
//                .cloned()
//                .filter_map(|ref m| m.checked_div(rhs))
//                .collect(),
//        };
//        if result.monomials.len() != self.monomials.len() {
//            None
//        } else {
//            Some(result)
//        }
//    }
//

impl<'a, 'b, I, C, P> Div<&'b Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        match self.checked_div(&rhs.into()) {
            Some(result) => result,
            None => panic!("Trying to divide {} by {} which is not possible.", self, rhs),
        }
    }
}

impl<'b, I, C, P> Div<&'b Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<'a, I, C, P> Div<Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.div(&rhs)
    }
}

impl<I, C, P> Div<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).div(&rhs)
    }
}


impl<'a, I, C, P> DivAssign<&'a Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

impl<I, C, P> DivAssign<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: Monomial<I, C, P>) {
        self.div_assign(&rhs)
    }
}

// impl<'a, I, C, P> CheckedDiv<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//    type Output = Polynomial<I, C, P>;
// impl<I, C, P> Polynomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//
//

impl<'a, 'b, I, C, P> Div<&'b Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        match self.checked_div(rhs) {
            Some(result) => result,
            None => panic!("Trying to divide {} by {} which is not possible.", self, rhs),
        }
    }
}

impl<'b, I, C, P> Div<&'b Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<'a, I, C, P> Div<Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.div(&rhs)
    }
}

impl<I, C, P> Div<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn div(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl<'a, I, C, P> DivAssign<&'a Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: &'a Polynomial<I, C, P>) {
        self.monomials = ((self as &Polynomial<I, C, P>) / rhs).monomials;
    }
}

impl<I, C, P> DivAssign<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: Polynomial<I, C, P>) {
        self.div_assign(&rhs)
    }
}

impl<'a, I, C, P> Neg for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn neg(self) -> Self::Output {
        Polynomial { monomials: self.monomials.iter().cloned().map(|ref x| -x).collect() }
    }
}

impl<I, C, P> Neg for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl<I, C, P> AddAssign<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: C) {
        if rhs != C::zero() {
            let mut remove: bool = false;
            if let Some(ref mut l) = self.monomials.last_mut() {
                if l.is_constant() {
                    if l.coefficient != -rhs.clone() {
                        l.coefficient += rhs;
                        return;
                    } else {
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

impl<I, C, P> Add<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: C) -> Self::Output {
        (&self).add(rhs)
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

impl<I, C, P> AddAssign<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: Monomial<I, C, P>) {
        self.add_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Add<&'b Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'b, I, C, P> Add<&'b Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<'a, I, C, P> Add<Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<I, C, P> Add<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).add(&rhs)
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
                i2 += 1;
            }
        }
        while i2 < rhs.monomials.len() {
            self.monomials.push(rhs.monomials[i2].clone());
            i2 += 1;
        }
    }
}

impl<I, C, P> AddAssign<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn add_assign(&mut self, rhs: Polynomial<I, C, P>) {
        self.add_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Add<&'b Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'b, I, C, P> Add<&'b Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<'a, I, C, P> Add<Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<I, C, P> Add<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).add(&rhs)
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

impl<'a, I, C, P> Sub<C> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: C) -> Self::Output {
        (&self).sub(rhs)
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

impl<I, C, P> SubAssign<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn sub_assign(&mut self, rhs: Monomial<I, C, P>) {
        self.sub_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Sub<&'b Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'b, I, C, P> Sub<&'b Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<'a, I, C, P> Sub<Monomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<I, C, P> Sub<Monomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).sub(&rhs)
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

impl<I, C, P> SubAssign<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    fn sub_assign(&mut self, rhs: Polynomial<I, C, P>) {
        self.sub_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Sub<&'b Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'b, I, C, P> Sub<&'b Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<'a, I, C, P> Sub<Polynomial<I, C, P>> for &'a Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<I, C, P> Sub<Polynomial<I, C, P>> for Polynomial<I, C, P>
    where I: Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).sub(&rhs)
    }
}
