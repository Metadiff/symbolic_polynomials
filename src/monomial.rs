use std::ops::{MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::convert::From;
use std::iter;

use traits::*;
use polynomial::Polynomial;

/// A symbolic monomial over the integers.
#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial {
    pub coefficient : i64,
    pub powers : Vec<(u16, u8)>
}

impl Monomial{
    /// Returns `true` if the two monomials differ only by a constant factor.
    ///
    /// # Examples
    /// ```
    /// # use symints::*;
    /// let mut registry = symints::Registry::default();
    /// let a = registry.new_monomial_variable();
    /// let b = registry.new_monomial_variable();
    /// let a_times_2 = 2 * &a;
    /// assert!(a.up_to_coefficient(&a_times_2));
    /// assert!(!a.up_to_coefficient(&b));
    /// ```
    pub fn up_to_coefficient(&self, other: &Monomial) -> bool {
        match self.powers.len() == other.powers.len() {
            true => {
                for (&(id, power), &(o_id, o_power)) in self.powers.iter().zip(other.powers.iter()) {
                    if id != o_id || power != o_power {
                        return false
                    }
                }
                true
            },
            false => false
        }
    }
}

impl IsConstant for Monomial {
    fn is_constant(&self) -> bool {
        self.powers.len() == 0
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.coefficient {
            0 => {
                return write!(f, "0")
            },
            1 => {
                if self.powers.len() == 0 {
                    return write!(f, "1")
                }
            },
            _ => {
                if self.coefficient < 0 {
                    try!(write!(f, "- {}", -self.coefficient))
                } else {
                    try!(write!(f, "{}", self.coefficient))
                }
            }
        }
        for &(id, pow) in self.powers.iter() {
            match pow {
                0 => {},
                1 => try!(write!(f, "{}", (id as u8 + ('a' as u8)) as char)),
                _ => try!(write!(f, "{}^{}", (id as u8 + ('a' as u8)) as char, pow))
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.coefficient {
            0 => {
                return write!(f, "0")
            },
            _ => {
                if self.coefficient < 0 {
                    try!(write!(f, "- {}", -self.coefficient))
                } else {
                    try!(write!(f, "{}", self.coefficient))
                }
            }
        }
        for &(id, pow) in self.powers.iter() {
            match pow {
                0 => {},
                _ => {
                    let id = (id as u8 + ('a' as u8)) as char;
                    let lower_id: String = id.to_lowercase().collect();
                    let repeated = iter::repeat(lower_id)
                        .take(pow as usize)
                        .collect::<Vec<String>>()
                        .join("*");
                    try!(write!(f, "*{}",  repeated))
                }
            }
        }
        Ok(())
    }
}

impl<C> From<C> for Monomial where C: Clone + Into<i64>{
    fn from(c: C) -> Self{
        Monomial{coefficient: c.into(), powers: Vec::new()}
    }
}

impl PartialEq for Monomial{
    fn eq(&self, other: &Monomial) -> bool {
        match self.coefficient == other.coefficient {
            true => self.up_to_coefficient(other),
            false => false
        }
    }
}

impl<C> PartialEq<C> for Monomial where C: Clone + Into<i64> {
    fn eq(&self, c: &C) -> bool {
        let other: i64 = (*c).clone().into();
        match other == self.coefficient {
            true => self.is_constant(),
            false => false
        }
    }
}

impl PartialEq<Polynomial> for Monomial {
    fn eq(&self, other: &Polynomial) -> bool {
        other.eq(self)
    }
}

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Monomial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> PartialOrd<C> for Monomial where C: Clone + Into<i64> {
    fn partial_cmp(&self, c: &C) -> Option<Ordering> {
        let other: i64 = (*c).clone().into();
        if self.is_constant() {
            match other.partial_cmp(&(self.coefficient)) {
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Equal) => Some(Ordering::Equal),
                Some(Ordering::Less) => Some(Ordering::Greater),
                None => None
            }
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Monomial) -> Ordering {
        let min = min(self.powers.len(), other.powers.len());
        for i in 0..min{
            match Ord::cmp(&self.powers[i].0, &other.powers[i].0){
                Ordering::Less => return Ordering::Greater,
                Ordering::Greater => return Ordering::Less,
                Ordering::Equal => match Ord::cmp(&self.powers[i].1,  &other.powers[i].1){
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
            }
        }
        match Ord::cmp(&self.powers.len(), &other.powers.len()){
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => Ord::cmp(&self.coefficient, &other.coefficient)
        }
    }
}

impl<C> MulAssign<C> for Monomial where C: Clone + Into<i64> {
    fn mul_assign(&mut self, rhs: C){
        self.coefficient *= rhs.into();
    }
}

impl<'a, C> Mul<C> for &'a Monomial where C: Clone + Into<i64> {
    type Output = Monomial;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a> MulAssign<&'a Monomial> for Monomial {
    fn mul_assign(&mut self, rhs: &'a Monomial){
        self.coefficient *= rhs.coefficient.clone();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.powers.len() && i2 < rhs.powers.len() {
            match Ord::cmp(&self.powers[i1].0, &rhs.powers[i2].0) {
                Ordering::Less => {}
                Ordering::Greater => {
                    self.powers.insert(i1, rhs.powers[i2]);
                    i2 += 1;
                }
                Ordering::Equal => {
                    self.powers[i1] = (self.powers[i1].0, self.powers[i1].1 + rhs.powers[i2].1);
                    i2 += 1;
                }
            }
            i1 += 1;
        }
        while i2 < rhs.powers.len() {
            self.powers.push(rhs.powers[i2]);
            i2 += 1;
        }
    }
}

impl<'a, 'b> Mul<&'a Monomial> for &'b Monomial{
    type Output = Monomial;
    fn mul(self, rhs: &'a Monomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, 'b> Mul<&'a Polynomial> for &'b Monomial{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
        result
    }
}

impl<C> CheckedDiv<C> for Monomial where C: Clone + Into<i64>{
    type Output = Monomial;
    fn checked_div(&self, c: C) -> Option<Self::Output> {
        let rhs: i64 = c.into();
        match self.coefficient.checked_rem(rhs) {
            Some(0) => Some(Monomial {
                coefficient: self.coefficient / rhs,
                powers: self.powers.clone()
            }),
            _ => None
        }
    }
}

impl<'a, C> Div<C> for &'a Monomial where C: Clone + Into<i64> {
    type Output = Monomial;
    fn div(self, c: C) -> Self::Output {
        self.checked_div(c).unwrap()
    }
}

impl<C> DivAssign<C> for Monomial where C: Clone + Into<i64> {
    fn div_assign(&mut self, c: C){
        let rhs: i64 = c.into();
        match self.coefficient.checked_rem(rhs) {
            Some(0) => {self.coefficient /= rhs},
            _ => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a> CheckedDiv<&'a Monomial> for Monomial {
    type Output = Monomial;
    fn checked_div(&self, rhs: &'a Monomial) -> Option<Self::Output> {
        match self.coefficient.checked_rem(rhs.coefficient.clone()) {
            Some(0) => {
                let mut result = Monomial{
                    coefficient : self.coefficient / rhs.coefficient,
                    powers: self.powers.clone()};
                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < result.powers.len() && i2 < rhs.powers.len() {
                    match Ord::cmp(&result.powers[i1].0, &rhs.powers[i2].0) {
                        Ordering::Greater => return None,
                        Ordering::Less => {i1 += 1;},
                        Ordering::Equal => {
                            match Ord::cmp(&result.powers[i1].1, &rhs.powers[i2].1){
                                Ordering::Less => return None,
                                Ordering::Equal => {result.powers.remove(i1); i2 += 1;},
                                Ordering::Greater => {
                                    result.powers[i1] = (result.powers[i1].0, result.powers[i1].1 - rhs.powers[i2].1);
                                    i1 += 1;
                                    i2 += 1;
                                }
                            }
                        }
                    }
                }
                if i2 < rhs.powers.len() {
                    None
                } else {
                    Some(result)
                }
            }
            _ => None
        }
    }
}

//impl CheckedDiv<Monomial> for Monomial {
//    type Output = Monomial;
//    fn checked_div(&self, rhs: Monomial) -> Option<Self::Output> {
//        self.checked_div(&rhs)
//    }
//}

impl<'a, 'b> Div<&'a Monomial> for &'b Monomial {
    type Output = Monomial;
    fn div(self, rhs: &'a Monomial) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a> DivAssign<&'a Monomial> for Monomial {
    fn div_assign(&mut self, rhs: &'a Monomial){
        let result = (self as &Monomial).checked_div(rhs).unwrap();
        self.coefficient = result.coefficient;
        self.powers = result.powers.clone();
    }
}

impl<'a> Neg for &'a Monomial {
    type Output = Monomial;
    fn neg(self) -> Self::Output {
        Monomial{coefficient: - self.coefficient, powers: self.powers.clone()}
    }
}

impl<'a, C> Add<C> for &'a Monomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn add(self, c: C) -> Self::Output {
        let rhs: i64 = c.into();
        if rhs == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == -self.coefficient {
                Polynomial::default()
            } else {
                let mut result = Polynomial::from(self);
                result.monomials[0].coefficient += rhs;
                result
            }
        } else {
            Polynomial{monomials: vec![self.clone(), Monomial::from(rhs)]}
        }
    }
}

impl<'a, 'b> Add<&'b Monomial> for &'a Monomial{
    type Output = Polynomial;
    fn add(self, rhs: &'b Monomial) -> Self::Output {
        if rhs.coefficient == 0 && self.coefficient == 0 {
            Polynomial::default()
        } else if rhs.coefficient == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            Polynomial{monomials: vec![rhs.clone()]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == -rhs.coefficient {
                Polynomial::default()
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient += rhs.coefficient;
                result
            }
        } else if self > rhs {
            Polynomial{monomials: vec![self.clone(), rhs.clone()]}
        } else {
            Polynomial{monomials: vec![rhs.clone(), self.clone()]}
        }
    }
}

impl<'a, 'b> Add<&'a Polynomial> for &'b Monomial {
    type Output = Polynomial;
    fn add(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
        result
    }
}

impl<'a, C> Sub<C> for &'a Monomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn sub(self, c: C) -> Self::Output {
        let rhs: i64 = c.into();
        if rhs == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == self.coefficient {
                Polynomial::default()
            } else {
                let mut result = Polynomial::from(self);
                result.monomials[0].coefficient -= rhs;
                result
            }
        } else {
            Polynomial{monomials: vec![self.clone(), Monomial::from(-rhs)]}
        }
    }
}

impl<'a, 'b> Sub<&'b Monomial> for &'a Monomial{
    type Output = Polynomial;
    fn sub(self, rhs: &'b Monomial) -> Self::Output {
        if self.coefficient == 0 && rhs.coefficient == 0 {
            Polynomial::default()
        } else if rhs.coefficient == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            Polynomial{monomials: vec![-rhs]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == rhs.coefficient {
                Polynomial::default()
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient -= rhs.coefficient;
                result
            }
        } else if self > rhs {
            Polynomial{monomials: vec![self.clone(), -rhs]}
        } else {
            Polynomial{monomials: vec![-rhs, self.clone()]}
        }
    }
}

impl<'a, 'b> Sub<&'a Polynomial> for &'b Monomial {
    type Output = Polynomial;
    fn sub(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = -rhs;
        result += self;
        result
    }
}
