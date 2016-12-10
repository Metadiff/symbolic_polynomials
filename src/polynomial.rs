use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::convert::From;

use traits::*;
use monomial::Monomial;

/// A symbolic polynomial over the integers.
#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial {
    pub monomials: Vec<Monomial>
}

impl IsConstant for Polynomial{
    fn is_constant(&self) -> bool {
        match self.monomials.len(){
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.monomials.len() {
            0 => write!(f, "0"),
            _ => {
                try!(write!(f, "{}", self.monomials[0]));
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > 0 {
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

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.monomials.len() {
            0 => write!(f, "0"),
            _ => {
                try!(write!(f, "{:?}", self.monomials[0]));
                for m in self.monomials.iter().skip(1) {
                    if m.coefficient > 0 {
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

impl<C> From<C> for Polynomial where C: Clone + Into<i64>{
    fn from(c: C) -> Self{
        Polynomial{monomials: vec![Monomial::from(c)]}
    }
}

impl<'a> From<&'a Monomial> for Polynomial{
    fn from(m: &'a Monomial) -> Self{
        Polynomial{monomials: vec![m.clone()]}
    }
}

impl From<Monomial> for Polynomial{
    fn from(m: Monomial) -> Self{
        Polynomial{monomials: vec![m]}
    }
}

//impl From<Monomial> for Polynomial{
//    fn from(m: Monomial) -> Self{
//        Polynomial{monomials: vec![m]}
//    }
//}

impl PartialEq for Polynomial{
    fn eq(&self, other: &Polynomial) -> bool {
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

impl<C> PartialEq<C> for Polynomial where C: Clone + Into<i64> {
    fn eq(&self, other: &C) -> bool {
        match self.monomials.len(){
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl PartialOrd for Polynomial {
    fn partial_cmp(&self, other: &Polynomial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> PartialOrd<C> for Polynomial where C: Clone + Into<i64> {
    fn partial_cmp(&self, c: &C) -> Option<Ordering> {
        let other: i64 = (*c).clone().into();
        match self.monomials.len() {
            0 => match other.partial_cmp(&0) {
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Equal) => Some(Ordering::Equal),
                Some(Ordering::Less) => Some(Ordering::Greater),
                None => None
            },
            1 => self.monomials[0].partial_cmp(&other),
            _ => Some(Ordering::Greater)
        }
    }
}

impl Ord for Polynomial {
    fn cmp(&self, other: &Polynomial) -> Ordering {
        let m = min(self.monomials.len(), other.monomials.len());
        for i in 0..m{
            match Ord::cmp(&self.monomials[i], &other.monomials[i]){
                Ordering::Equal => {},
                result => return result
            }
        }
        Ord::cmp(&self.monomials.len(), &other.monomials.len())
    }
}

impl<C> MulAssign<C> for Polynomial where C: Clone + Into<i64> {
    fn mul_assign(&mut self, rhs: C){
        for m in self.monomials.iter_mut() {
            *m *= rhs.clone();
        }
    }
}

impl<'a, C> Mul<C> for &'a Polynomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a> MulAssign<&'a Monomial> for Polynomial {
    fn mul_assign(&mut self, rhs: &'a Monomial){
        for mut m in self.monomials.iter_mut() {
            *m *= rhs;
        }
    }
}

impl<'a, 'b> Mul<&'a Monomial> for &'b Polynomial{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Monomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a> MulAssign<&'a Polynomial> for Polynomial {
    fn mul_assign(&mut self, rhs: &'a Polynomial){
        match self.monomials.len() {
            0 => {},
            _ => {
                let mut result = Polynomial::default();
                for m in self.monomials.iter(){
                    result += &(m * rhs);
                }
                self.monomials = result.monomials;
            }
        }
    }
}

impl<'a, 'b> Mul<&'a Polynomial> for &'b Polynomial{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<C> CheckedDiv<C> for Polynomial where C: Clone + Into<i64> {
    type Output = Polynomial;
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

impl<'a, C> Div<C> for &'a Polynomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn div(self, rhs: C) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<C> DivAssign<C> for Polynomial where C: Clone + Into<i64> {
    fn div_assign(&mut self, rhs: C){
        self.monomials = ((self as &Polynomial) / rhs).monomials;
    }
}

impl<'a> CheckedDiv<&'a Monomial> for Polynomial {
    type Output = Polynomial;
    fn checked_div(&self, rhs: &'a Monomial) -> Option<Self::Output> {
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

impl<'a, 'b> Div<&'a Monomial> for &'b Polynomial {
    type Output = Polynomial;
    fn div(self, rhs: &'a Monomial) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a> DivAssign<&'a Monomial> for Polynomial {
    fn div_assign(&mut self, rhs: &'a Monomial){
        self.monomials = ((self as &Polynomial) / rhs).monomials;
    }
}

impl<'a> CheckedDiv<&'a Polynomial> for Polynomial {
    type Output = Polynomial;
    fn checked_div(&self, rhs: &'a Polynomial) -> Option<Self::Output> {
        let mut result = Polynomial::default();
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

impl<'a, 'b> Div<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;
    fn div(self, rhs: &'a Polynomial) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a> DivAssign<&'a Polynomial> for Polynomial {
    fn div_assign(&mut self, rhs: &'a Polynomial){
        self.monomials = ((self as &Polynomial) / rhs).monomials;
    }
}

impl<'a> Neg for &'a Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        Polynomial{monomials: self.monomials.iter().cloned().map(|ref x| -x).collect()}
    }
}

impl<C> AddAssign<C> for Polynomial where C: Clone + Into<i64> {
    fn add_assign(&mut self, c: C) {
        let rhs: i64 = c.into();
        if rhs != 0 {
            let mut remove: bool = false;
            match self.monomials.last_mut() {
                Some(ref mut l) => {
                    if l.is_constant() {
                        l.coefficient += rhs;
                        if l.coefficient == 0 {
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

impl<'a, C> Add<C> for &'a Polynomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn add(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a> AddAssign<&'a Monomial> for Polynomial {
    fn add_assign(&mut self, rhs: &'a Monomial) {
        if rhs.coefficient != 0 {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient += rhs.coefficient;
                    if self.monomials[i].coefficient == 0 {
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

impl<'a, 'b> Add<&'a Monomial> for &'b Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: &'a Monomial) -> Self::Output{
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a> AddAssign<&'a Polynomial> for Polynomial {
    fn add_assign(&mut self, rhs: &'a Polynomial) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]){
                self.monomials[i1].coefficient += rhs.monomials[i2].coefficient;
                if self.monomials[i1].coefficient == 0 {
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

impl<'a, 'b> Add<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<C> SubAssign<C> for Polynomial where C: Clone + Into<i64> {
    fn sub_assign(&mut self, c: C) {
        let rhs: i64 = c.into();
        if rhs != 0 {
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

impl<'a, C> Sub<C> for &'a Polynomial where C: Clone + Into<i64> {
    type Output = Polynomial;
    fn sub(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a> SubAssign<&'a Monomial> for Polynomial {
    fn sub_assign(&mut self, rhs: &'a Monomial) {
        if rhs.coefficient != 0 {
            for i in 0..self.monomials.len() {
                if self.monomials[i].up_to_coefficient(rhs) {
                    self.monomials[i].coefficient -= rhs.coefficient;
                    if self.monomials[i].coefficient == 0 {
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

impl<'a, 'b> Sub<&'a Monomial> for &'b Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: &'a Monomial) -> Self::Output{
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a> SubAssign<&'a Polynomial> for Polynomial {
    fn sub_assign(&mut self, rhs: &'a Polynomial) {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.monomials.len() && i2 < rhs.monomials.len() {
            if self.monomials[i1].up_to_coefficient(&rhs.monomials[i2]){
                self.monomials[i1].coefficient -= rhs.monomials[i2].coefficient;
                if self.monomials[i1].coefficient == 0 {
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

impl<'a, 'b> Sub<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}