use std::ops::{MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::result::Result;
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::convert::From;
use std::collections::HashMap;
use std::iter;

use primitives::{Monomial, Polynomial, IsConstant, Evaluable, CheckedDiv, Id};

impl<I> Monomial<I> where I: Id {
    pub fn up_to_coefficient(&self, other: &Monomial<I>) -> bool {
        match self.powers.len() == other.powers.len() {
            true => {
                for (&(ref c, power), &(ref o_c, o_power)) in self.powers.iter().zip(other.powers.iter()) {
                    if c != o_c || power != o_power {
                        return false
                    }
                }
                true
            },
            false => false
        }
    }
}

impl<I> IsConstant for Monomial<I> where I: Id {
    fn is_constant(&self) -> bool {
        self.powers.len() == 0
    }
}

impl<I> Evaluable<I> for Monomial<I> where I: Id {
    fn evaluate(&self, values: &HashMap<I, i64>) -> Result<i64, I> {
        let mut value = self.coefficient;
        for &(ref c, pow) in self.powers.iter(){
            value *= try!(c.evaluate(values)).pow(pow as u32);
        }
        Ok(value)
    }
}

impl<I> fmt::Display for Monomial<I> where I: Id {
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
        for &(ref c, pow) in self.powers.iter() {
            match pow {
                0 => {},
                1 => try!(write!(f, "{}", c)),
                v => try!(write!(f, "{}^{}", c, v))
            }
        }
        Ok(())
    }
}

impl<I> fmt::Debug for Monomial<I> where I: Id {
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
        for &(ref c, pow) in self.powers.iter() {
            match pow {
                0 => {},
                _ => {
                    let str = format!("{:?}", c);
                    let repeated = iter::repeat(str)
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

impl<I, C> From<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    fn from(c: C) -> Self{
        Monomial{coefficient: c.into(), powers: Vec::new()}
    }
}

impl<I, C> PartialEq<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    fn eq(&self, c: &C) -> bool {
        let other: i64 = (*c).clone().into();
        match self.coefficient == other {
            true => self.is_constant(),
            false => false
        }
    }
}

impl<I> PartialEq for Monomial<I> where I: Id {
    fn eq(&self, other: &Monomial<I>) -> bool {
        match self.coefficient == other.coefficient {
            true => self.up_to_coefficient(other),
            false => false
        }
    }
}

impl<I> PartialEq<Polynomial<I>> for Monomial<I> where I:Id {
    fn eq(&self, other: &Polynomial<I>) -> bool {
        other.eq(self)
    }
}

impl<I, C> PartialOrd<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        if self.is_constant() {
            self.coefficient.partial_cmp(&other.clone().into())
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<I> PartialOrd for Monomial<I> where I: Id {
    fn partial_cmp(&self, other: &Monomial<I>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I> PartialOrd<Polynomial<I>> for Monomial<I> where I: Id {
    fn partial_cmp(&self, other: &Polynomial<I>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<I> Ord for Monomial<I> where I: Id {
    fn cmp(&self, other: &Monomial<I>) -> Ordering {
        let min = min(self.powers.len(), other.powers.len());
        for i in 0..min{
            match Ord::cmp(&self.powers[i].0, &other.powers[i].0){
                Ordering::Equal => match Ord::cmp(&self.powers[i].1,  &other.powers[i].1){
                    Ordering::Equal => {},
                    v => return v
                },
                v => return v
            }
        }
        match Ord::cmp(&self.powers.len(), &other.powers.len()){
            Ordering::Equal => Ord::cmp(&self.coefficient, &other.coefficient),
            v => v
        }
    }
}

impl<I, C> MulAssign<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    fn mul_assign(&mut self, rhs: C){
        self.coefficient *= rhs.into();
    }
}

impl<'a, I, C> Mul<C> for &'a Monomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Monomial<I>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I> MulAssign<&'a Monomial<I>> for Monomial<I> where I: Id {
    fn mul_assign(&mut self, rhs: &'a Monomial<I>){
        self.coefficient *= rhs.coefficient.clone();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.powers.len() && i2 < rhs.powers.len() {
            match Ord::cmp(&self.powers[i1].0, &rhs.powers[i2].0) {
                Ordering::Greater => {}
                Ordering::Less => {
                    self.powers.insert(i1, rhs.powers[i2].clone());
                    i2 += 1;
                }
                Ordering::Equal => {
                    self.powers[i1] = (self.powers[i1].0.clone(), self.powers[i1].1 + rhs.powers[i2].1);
                    i2 += 1;
                }
            }
            i1 += 1;
        }
        while i2 < rhs.powers.len() {
            self.powers.push(rhs.powers[i2].clone());
            i2 += 1;
        }
    }
}

impl<'a, 'b, I> Mul<&'a Monomial<I>> for &'b Monomial<I> where I: Id {
    type Output = Monomial<I>;
    fn mul(self, rhs: &'a Monomial<I>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, 'b, I> Mul<&'a Polynomial<I>> for &'b Monomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn mul(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
        result
    }
}

impl<I, C> CheckedDiv<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Monomial<I>;
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

impl<'a, I, C> Div<C> for &'a Monomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Monomial<I>;
    fn div(self, c: C) -> Self::Output {
        self.checked_div(c).unwrap()
    }
}

impl<I, C> DivAssign<C> for Monomial<I> where I: Id, C: Clone + Into<i64> {
    fn div_assign(&mut self, c: C){
        let rhs: i64 = c.into();
        match self.coefficient.checked_rem(rhs) {
            Some(0) => {self.coefficient /= rhs},
            _ => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a, I> CheckedDiv<&'a Monomial<I>> for Monomial<I> where I: Id {
    type Output = Monomial<I>;
    fn checked_div(&self, rhs: &'a Monomial<I>) -> Option<Self::Output> {
        match self.coefficient.checked_rem(rhs.coefficient.clone()) {
            Some(0) => {
                let mut result = Monomial{
                    coefficient : self.coefficient / rhs.coefficient,
                    powers: self.powers.clone()};
                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < result.powers.len() && i2 < rhs.powers.len() {
                    match Ord::cmp(&result.powers[i1].0, &rhs.powers[i2].0) {
                        Ordering::Less => return None,
                        Ordering::Greater => {i1 += 1;},
                        Ordering::Equal => {
                            match Ord::cmp(&result.powers[i1].1, &rhs.powers[i2].1){
                                Ordering::Less => return None,
                                Ordering::Equal => {result.powers.remove(i1); i2 += 1;},
                                Ordering::Greater => {
                                    result.powers[i1] = (result.powers[i1].0.clone(), result.powers[i1].1 - rhs.powers[i2].1);
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

impl<'a, 'b, I> Div<&'a Monomial<I>> for &'b Monomial<I> where I: Id {
    type Output = Monomial<I>;
    fn div(self, rhs: &'a Monomial<I>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I> DivAssign<&'a Monomial<I>> for Monomial<I> where I: Id {
    fn div_assign(&mut self, rhs: &'a Monomial<I>){
        let result = (self as &Monomial<I>).checked_div(rhs).unwrap();
        self.coefficient = result.coefficient;
        self.powers = result.powers.clone();
    }
}

impl<'a, I> Neg for &'a Monomial<I> where I: Id {
    type Output = Monomial<I>;
    fn neg(self) -> Self::Output {
        Monomial{coefficient: - self.coefficient, powers: self.powers.clone()}
    }
}

impl<'a, I, C> Add<C> for &'a Monomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn add(self, c: C) -> Self::Output {
        let rhs: i64 = c.into();
        if rhs == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == -self.coefficient {
                Polynomial{monomials: Vec::new()}
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

impl<'a, 'b, I> Add<&'b Monomial<I>> for &'a Monomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn add(self, rhs: &'b Monomial<I>) -> Self::Output {
        if rhs.coefficient == 0 && self.coefficient == 0 {
            Polynomial{monomials: Vec::new()}
        } else if rhs.coefficient == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            Polynomial{monomials: vec![rhs.clone()]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == -rhs.coefficient {
                Polynomial{monomials: Vec::new()}
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

impl<'a, 'b, I> Add<&'a Polynomial<I>> for &'b Monomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn add(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
        result
    }
}

impl<'a, I, C> Sub<C> for &'a Monomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn sub(self, c: C) -> Self::Output {
        let rhs: i64 = c.into();
        if rhs == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == self.coefficient {
                Polynomial{monomials: Vec::new()}
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

impl<'a, 'b, I> Sub<&'b Monomial<I>> for &'a Monomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'b Monomial<I>) -> Self::Output {
        if self.coefficient == 0 && rhs.coefficient == 0 {
            Polynomial{monomials: Vec::new()}
        } else if rhs.coefficient == 0 {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            Polynomial{monomials: vec![-rhs]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == rhs.coefficient {
                Polynomial{monomials: Vec::new()}
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

impl<'a, 'b, I> Sub<&'a Polynomial<I>> for &'b Monomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = -rhs;
        result += self;
        result
    }
}
