use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::convert::From;

use primitives::*;

impl<I, C> IsConstant for Polynomial<I, C> where I: Id, C: Coefficient {
    fn is_constant(&self) -> bool {
        match self.monomials.len(){
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }
}

impl<I, C> Evaluable<I, C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn evaluate(&self, values: &HashMap<I, C>) -> Result<C, I> {
        let mut value = C::zero();
        for m in self.monomials.iter(){
            value += try!(m.evaluate(values));
        }
        Ok(value)
    }
}

impl<I, C> fmt::Display for Polynomial<I, C> where I: Id, C: Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.monomials.len() {
            0 => write!(f, "0"),
            _ => {
                try!(write!(f, "{}", self.monomials[0]));
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > C::zero() {
                        try!(write!(f, " + {}", m));
                    } else {
                        try!(write!(f, " {}", m));
                    }
                }
                Ok(())
            }
        }
    }
}

impl<I, C> fmt::Debug for Polynomial<I, C> where I: Id, C: Coefficient {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.monomials.len() {
            0 => write!(f, "0"),
            _ => {
                try!(write!(f, "{:?}", self.monomials[0]));
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > C::zero() {
                        try!(write!(f, " + {:?}", m));
                    } else {
                        try!(write!(f, " {:?}", m));
                    }
                }
                Ok(())
            }
        }
    }
}

impl<I, C> From<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn from(other: C) -> Self{
        Polynomial{monomials: vec![Monomial::from(other)]}
    }
}

impl<'a, I, C> From<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn from(m: &'a Monomial<I, C>) -> Self{
        Polynomial{monomials: vec![m.clone()]}
    }
}

impl<I, C> From<Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn from(m: Monomial<I, C>) -> Self{
        Polynomial{monomials: vec![m]}
    }
}

impl<I, C> PartialEq<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn eq(&self, other: &C) -> bool {
        match self.monomials.len(){
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I, C> PartialEq<Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn eq(&self, other: &Monomial<I, C>) -> bool {
        match self.monomials.len(){
            0 => other.coefficient == C::zero(),
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I, C> PartialEq for Polynomial<I, C> where I: Id, C: Coefficient {
    fn eq(&self, other: &Polynomial<I, C>) -> bool {
        match self.monomials.len() == other.monomials.len() {
            false => false,
            true => {
                for (ms, mo) in self.monomials.iter().zip(other.monomials.iter()) {
                    if ms != mo {
                        return false
                    }
                }
                true
            }
        }
    }
}

impl<I, C> PartialOrd<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        match self.monomials.len() {
            0 => match other.partial_cmp(&C::zero()) {
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Equal) => Some(Ordering::Equal),
                Some(Ordering::Less) => Some(Ordering::Greater),
                None => None
            },
            1 => self.monomials[0].partial_cmp(other),
            _ => Some(Ordering::Greater)
        }
    }
}

impl<I, C> PartialOrd<Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn partial_cmp(&self, other: &Monomial<I, C>) -> Option<Ordering> {
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

impl<I, C> PartialOrd for Polynomial<I, C> where I: Id, C: Coefficient {
    fn partial_cmp(&self, other: &Polynomial<I, C>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C> Ord for Polynomial<I, C> where I: Id, C: Coefficient {
    fn cmp(&self, other: &Polynomial<I, C>) -> Ordering {
        let m = if self.monomials.len() < other.monomials.len() {self.monomials.len() } else {other.monomials.len()};
        for i in 0..m{
            match Ord::cmp(&self.monomials[i], &other.monomials[i]){
                Ordering::Equal => {},
                result => return result
            }
        }
        Ord::cmp(&self.monomials.len(), &other.monomials.len())
    }
}

impl<I, C> MulAssign<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn mul_assign(&mut self, rhs: C){
        for m in self.monomials.iter_mut() {
            *m *= rhs.clone();
        }
    }
}

impl<'a, I, C> Mul<C> for &'a Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C> MulAssign<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn mul_assign(&mut self, rhs: &'a Monomial<I, C>){
        for mut m in self.monomials.iter_mut() {
            *m *= rhs;
        }
    }
}

impl<'a, 'b, I, C> Mul<&'a Monomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn mul(self, rhs: &'a Monomial<I, C>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C> MulAssign<&'a Polynomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn mul_assign(&mut self, rhs: &'a Polynomial<I, C>){
        match self.monomials.len() {
            0 => {},
            _ => {
                let mut result = Polynomial{monomials: Vec::new()};
                for m in self.monomials.iter(){
                    result += &(m * rhs);
                }
                self.monomials = result.monomials;
            }
        }
    }
}

impl<'a, 'b, I, C> Mul<&'a Polynomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn mul(self, rhs: &'a Polynomial<I, C>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<I, C> CheckedDiv<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn checked_div(&self, rhs: C) -> Option<Self::Output> {
        let result = Polynomial{monomials: self.monomials.iter()
            .cloned()
            .filter_map(|ref m| m.checked_div(rhs.clone()))
            .collect()};
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, I, C> Div<C> for &'a Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn div(self, rhs: C) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<I, C> DivAssign<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn div_assign(&mut self, rhs: C){
        self.monomials = ((self as &Polynomial<I, C>) / rhs).monomials;
    }
}

impl<'a, I, C> CheckedDiv<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn checked_div(&self, rhs: &'a Monomial<I, C>) -> Option<Self::Output> {
        let result = Polynomial{monomials: self.monomials.iter()
            .cloned()
            .filter_map(|ref m| m.checked_div(rhs))
            .collect()};
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, 'b, I, C> Div<&'a Monomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn div(self, rhs: &'a Monomial<I, C>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C> DivAssign<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn div_assign(&mut self, rhs: &'a Monomial<I, C>){
        self.monomials = ((self as &Polynomial<I, C>) / rhs).monomials;
    }
}

impl<'a, I, C> CheckedDiv<&'a Polynomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn checked_div(&self, rhs: &'a Polynomial<I, C>) -> Option<Self::Output> {
        let mut result = Polynomial{monomials: Vec::new()};
        let mut reminder = self.clone();
        while ! reminder.is_constant() {
            match (reminder.monomials[0]).checked_div(&rhs.monomials[0]) {
                Some(ref x) => {
                    result += x;
                    reminder -= &(rhs * x);
                },
                None => return None
            }
        }
        if reminder.monomials.len() > 0 {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, 'b, I, C> Div<&'a Polynomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn div(self, rhs: &'a Polynomial<I, C>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C> DivAssign<&'a Polynomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn div_assign(&mut self, rhs: &'a Polynomial<I, C>){
        self.monomials = ((self as &Polynomial<I, C>) / rhs).monomials;
    }
}

impl<'a, I, C> Neg for &'a Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn neg(self) -> Self::Output {
        Polynomial{monomials: self.monomials.iter().cloned().map(|ref x| -x).collect()}
    }
}

impl<I, C> AddAssign<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn add_assign(&mut self, rhs: C) {
        if rhs != C::zero() {
            let mut remove: bool = false;
            match self.monomials.last_mut() {
                Some(ref mut l) => {
                    if l.is_constant() {
                        l.coefficient += rhs.clone();
                        if l.coefficient == C::zero() {
                            remove = true;
                        }
                    }
                },
                None => {}
            }
            if remove {
                self.monomials.pop();
            } else {
                self.monomials.push(Monomial::from(rhs));
            }
        }
    }
}

impl<'a, I, C> Add<C> for &'a Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn add(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I, C> AddAssign<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn add_assign(&mut self, rhs: &'a Monomial<I, C>) {
        if rhs.coefficient != C::zero() {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient += rhs.coefficient.clone();
                    if self.monomials[i].coefficient == C::zero() {
                        self.monomials.remove(i);
                    }
                    return
                }
                match Ord::cmp(&(self.monomials[i]), rhs) {
                    Ordering::Less => {
                        self.monomials.insert(i, rhs.clone());
                        return
                    }
                    _ => {}
                }
            }
            self.monomials.push(rhs.clone());
        }
    }
}

impl<'a, 'b, I, C> Add<&'a Monomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn add(self, rhs: &'a Monomial<I, C>) -> Self::Output{
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I, C> AddAssign<&'a Polynomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn add_assign(&mut self, rhs: &'a Polynomial<I, C>) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]){
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

impl<'a, 'b, I, C> Add<&'a Polynomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn add(self, rhs: &'a Polynomial<I, C>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<I, C> SubAssign<C> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn sub_assign(&mut self, rhs: C) {
        if rhs != C::zero() {
            let mut remove: bool = false;
            match self.monomials.last_mut() {
                Some(ref mut l) => {
                    if l.is_constant() {
                        if rhs == l.coefficient {
                            remove = true;
                        } else {
                            l.coefficient -= rhs;
                            return
                        }
                    }
                },
                None => {}
            }
            if remove {
                self.monomials.pop();
            } else {
                self.monomials.push(Monomial::from(-rhs));
            }
        }
    }
}

impl<'a, I, C> Sub<C> for &'a Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn sub(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I, C> SubAssign<&'a Monomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient  {
    fn sub_assign(&mut self, rhs: &'a Monomial<I, C>) {
        if rhs.coefficient != C::zero() {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient -= rhs.coefficient.clone();
                    if self.monomials[i].coefficient == C::zero() {
                        self.monomials.remove(i);
                    }
                    return
                }
                match Ord::cmp(&(self.monomials[i]), rhs) {
                    Ordering::Less => {
                        self.monomials.insert(i, rhs.clone());
                        return
                    }
                    _ => {}
                }
            }
            self.monomials.push(rhs.clone());
        }
    }
}

impl<'a, 'b, I, C> Sub<&'a Monomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn sub(self, rhs: &'a Monomial<I, C>) -> Self::Output{
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I, C> SubAssign<&'a Polynomial<I, C>> for Polynomial<I, C> where I: Id, C: Coefficient {
    fn sub_assign(&mut self, rhs: &'a Polynomial<I, C>) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]){
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

impl<'a, 'b, I, C> Sub<&'a Polynomial<I, C>> for &'b Polynomial<I, C> where I: Id, C: Coefficient {
    type Output = Polynomial<I, C>;
    fn sub(self, rhs: &'a Polynomial<I, C>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}