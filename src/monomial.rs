use std::ops::{MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::result::Result;
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::convert::From;
use std::collections::HashMap;
use std::iter;

use primitives::*;
use num;

impl<I, C, P> Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    pub fn up_to_coefficient(&self, other: &Monomial<I, C, P>) -> bool {
        match self.powers.len() == other.powers.len() {
            true => {
                for (&(ref c, ref power), &(ref o_c, ref o_power)) in self.powers.iter().zip(other.powers.iter()) {
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

impl<I, C, P> IsConstant for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn is_constant(&self) -> bool {
        self.powers.len() == 0
    }
}

impl<I, C, P> Evaluable<I, C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn evaluate(&self, values: &HashMap<I, C>) -> Result<C, I> {
        let mut value = self.coefficient.clone();
        for &(ref c, ref pow) in self.powers.iter(){
            value *= num::pow(try!(c.evaluate(values)), pow.clone().into());
        }
        Ok(value)
    }
}

impl<I, C, P> fmt::Display for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.coefficient == C::zero() {
            return write!(f, "0")
        } else if self.coefficient == C::one() && self.powers.len() == 0 {
            return write!(f, "1")
        } else {
            if self.coefficient < C::zero() {
                try!(write!(f, "- {}", - self.coefficient.clone()))
            } else {
                try!(write!(f, "{}", self.coefficient.clone()))
            }
        }
        for &(ref c, ref pow) in self.powers.iter() {
            if pow == &P::one() {
                try!(write!(f, "{}", c))
            } else if pow != &P::zero() {
                try!(write!(f, "{}^{}", c, pow))
            }
        }
        Ok(())
    }
}

impl<I, C, P> fmt::Debug for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.coefficient == C::zero() {
            return write!(f, "0")
        } else {
            if self.coefficient < C::zero() {
                try!(write!(f, "- {}", -self.coefficient.clone()))
            } else {
                try!(write!(f, "{}", self.coefficient.clone()))
            }
        }
        for &(ref c, ref pow) in self.powers.iter() {
            if pow != &P::zero() {
                let str = format!("{:?}", c);
                let repeated = iter::repeat(str)
                    .take(pow.clone().into())
                    .collect::<Vec<String>>()
                    .join("*");
                try!(write!(f, "*{}",  repeated))
            }
        }
        Ok(())
    }
}

impl<I, C, P> From<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn from(other: C) -> Self{
        Monomial{coefficient: other, powers: Vec::new()}
    }
}

impl<I, C, P> PartialEq<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn eq(&self, other: &C) -> bool {
        match self.coefficient == *other {
            true => self.is_constant(),
            false => false
        }
    }
}

impl<I, C, P> PartialEq for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn eq(&self, other: &Monomial<I, C, P>) -> bool {
        match self.coefficient == other.coefficient {
            true => self.up_to_coefficient(other),
            false => false
        }
    }
}

impl<I, C, P> PartialEq<Polynomial<I, C, P>> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn eq(&self, other: &Polynomial<I, C, P>) -> bool {
        other.eq(self)
    }
}

impl<I, C, P> PartialOrd<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        if self.is_constant() {
            self.coefficient.partial_cmp(&other.clone().into())
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<I, C, P> PartialOrd for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &Monomial<I, C, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C, P> PartialOrd<Polynomial<I, C, P>> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn partial_cmp(&self, other: &Polynomial<I, C, P>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<I, C, P> Ord for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn cmp(&self, other: &Monomial<I, C, P>) -> Ordering {
        let min = if self.powers.len() < other.powers.len() {self.powers.len()} else {other.powers.len()};
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

impl<I, C, P> MulAssign<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: C){
        self.coefficient *= rhs;
    }
}

impl<'a, I, C, P> Mul<C> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C, P> MulAssign<&'a Monomial<I, C, P>> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn mul_assign(&mut self, rhs: &'a Monomial<I, C, P>){
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
                    self.powers[i1] = (self.powers[i1].0.clone(),
                                       self.powers[i1].1.clone() + rhs.powers[i2].1.clone());
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

impl<'a, 'b, I, C, P> Mul<&'a Monomial<I, C, P>> for &'b Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, 'b, I, C, P> Mul<&'a Polynomial<I, C, P>> for &'b Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
        result
    }
}

impl<I, C, P> CheckedDiv<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn checked_div(&self, other: C) -> Option<Self::Output> {
        let (d, rem) = self.coefficient.div_rem(&other);
        if rem == C::zero() {
            Some(Monomial {
                coefficient: d,
                powers: self.powers.clone()
            })
        } else {
            None
        }
    }
}

impl<'a, I, C, P> Div<C> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, other: C) -> Self::Output {
        self.checked_div(other).unwrap()
    }
}

impl<I, C, P> DivAssign<C> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: C){
        let (d, rem) = self.coefficient.div_rem(&rhs);
        if rem == C::zero() {
            self.coefficient = d;
        } else {
            panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a, I, C, P> CheckedDiv<&'a Monomial<I, C, P>> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn checked_div(&self, rhs: &'a Monomial<I, C, P>) -> Option<Self::Output> {
        let (d, rem) = self.coefficient.div_rem(&rhs.coefficient);
        if rem == C::zero() {
            let mut result = Monomial{
                coefficient : d,
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
                                result.powers[i1] = (result.powers[i1].0.clone(),
                                                     result.powers[i1].1.clone() - rhs.powers[i2].1.clone());
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
            else {
                None
            }
    }
}

impl<'a, 'b, I, C, P> Div<&'a Monomial<I, C, P>> for &'b Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, rhs: &'a Monomial<I, C, P>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C, P> DivAssign<&'a Monomial<I, C, P>> for Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    fn div_assign(&mut self, rhs: &'a Monomial<I, C, P>){
        let result = (self as &Monomial<I, C, P>).checked_div(rhs).unwrap();
        self.coefficient = result.coefficient;
        self.powers = result.powers.clone();
    }
}

impl<'a, I, C, P> Neg for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Monomial<I, C, P>;
    fn neg(self) -> Self::Output {
        Monomial{coefficient: - self.coefficient.clone(), powers: self.powers.clone()}
    }
}

impl<'a, I, C, P> Add<C> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: C) -> Self::Output {
        if rhs == C::zero() {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == -self.coefficient.clone() {
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

impl<'a, 'b, I, C, P> Add<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        if rhs.coefficient == C::zero() && self.coefficient == C::zero() {
            Polynomial{monomials: Vec::new()}
        } else if rhs.coefficient == C::zero() {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == C::zero() {
            Polynomial{monomials: vec![rhs.clone()]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == -rhs.coefficient.clone() {
                Polynomial{monomials: Vec::new()}
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient += rhs.coefficient.clone();
                result
            }
        } else if self > rhs {
            Polynomial{monomials: vec![self.clone(), rhs.clone()]}
        } else {
            Polynomial{monomials: vec![rhs.clone(), self.clone()]}
        }
    }
}

impl<'a, 'b, I, C, P> Add<&'a Polynomial<I, C, P>> for &'b Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
        result
    }
}

impl<'a, I, C, P> Sub<C> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: C) -> Self::Output {
        if rhs == C::zero() {
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

impl<'a, 'b, I, C, P> Sub<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        if self.coefficient == C::zero() && rhs.coefficient == C::zero() {
            Polynomial{monomials: Vec::new()}
        } else if rhs.coefficient == C::zero() {
            Polynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == C::zero() {
            Polynomial{monomials: vec![-rhs]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == rhs.coefficient {
                Polynomial{monomials: Vec::new()}
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient -= rhs.coefficient.clone();
                result
            }
        } else if self > rhs {
            Polynomial{monomials: vec![self.clone(), -rhs]}
        } else {
            Polynomial{monomials: vec![-rhs, self.clone()]}
        }
    }
}

impl<'a, 'b, I, C, P> Sub<&'a Polynomial<I, C, P>> for &'b Monomial<I, C, P> where I:Id, C: Coefficient, P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = -rhs;
        result += self;
        result
    }
}
