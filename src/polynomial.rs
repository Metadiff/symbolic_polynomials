use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::collections::HashMap;
use std::convert::From;

use primitives::{Monomial, Polynomial, IsConstant, Evaluable, CheckedDiv, Id};

impl<I> IsConstant for Polynomial<I> where I: Id {
    fn is_constant(&self) -> bool {
        match self.monomials.len(){
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }
}

impl<I> Evaluable<I> for Polynomial<I> where I: Id {
    fn evaluate(&self, values: &HashMap<I, i64>) -> Result<i64, I> {
        let mut value = 0;
        for m in self.monomials.iter(){
            value += try!(m.evaluate(values));
        }
        Ok(value)
    }
}

impl<I> fmt::Display for Polynomial<I> where I: Id {
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

impl<I> fmt::Debug for Polynomial<I> where I: Id {
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

impl<I, C> From<C> for Polynomial<I> where I: Id, C: Clone + Into<i64>{
    fn from(c: C) -> Self{
        Polynomial{monomials: vec![Monomial::from(c)]}
    }
}

impl<'a, I> From<&'a Monomial<I>> for Polynomial<I> where I: Id {
    fn from(m: &'a Monomial<I>) -> Self{
        Polynomial{monomials: vec![m.clone()]}
    }
}

impl<I> From<Monomial<I>> for Polynomial<I> where I: Id {
    fn from(m: Monomial<I>) -> Self{
        Polynomial{monomials: vec![m]}
    }
}

impl<I, C> PartialEq<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
    fn eq(&self, other: &C) -> bool {
        match self.monomials.len(){
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I> PartialEq<Monomial<I>> for Polynomial<I> where I: Id {
    fn eq(&self, other: &Monomial<I>) -> bool {
        match self.monomials.len(){
            0 => other.coefficient == 0,
            1 => self.monomials[0].eq(other),
            _ => false
        }
    }
}

impl<I> PartialEq for Polynomial<I> where I: Id {
    fn eq(&self, other: &Polynomial<I>) -> bool {
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

impl<I, C> PartialOrd<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
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

impl<I> PartialOrd<Monomial<I>> for Polynomial<I> where I: Id {
    fn partial_cmp(&self, other: &Monomial<I>) -> Option<Ordering> {
        match self.monomials.len() {
            0 => {
                if other.is_constant() {
                    0.partial_cmp(&other.coefficient)
                } else {
                    Some(Ordering::Less)
                }
            }
            1 => self.monomials[0].partial_cmp(other),
            _ => Some(Ordering::Greater)
        }
    }
}

impl<I> PartialOrd for Polynomial<I> where I: Id {
    fn partial_cmp(&self, other: &Polynomial<I>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I> Ord for Polynomial<I> where I: Id {
    fn cmp(&self, other: &Polynomial<I>) -> Ordering {
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

impl<I, C> MulAssign<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
    fn mul_assign(&mut self, rhs: C){
        for m in self.monomials.iter_mut() {
            *m *= rhs.clone();
        }
    }
}

impl<'a, I, C> Mul<C> for &'a Polynomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I> MulAssign<&'a Monomial<I>> for Polynomial<I> where I: Id {
    fn mul_assign(&mut self, rhs: &'a Monomial<I>){
        for mut m in self.monomials.iter_mut() {
            *m *= rhs;
        }
    }
}

impl<'a, 'b, I> Mul<&'a Monomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn mul(self, rhs: &'a Monomial<I>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I> MulAssign<&'a Polynomial<I>> for Polynomial<I> where I: Id {
    fn mul_assign(&mut self, rhs: &'a Polynomial<I>){
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

impl<'a, 'b, I> Mul<&'a Polynomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn mul(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<I, C> CheckedDiv<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
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

impl<'a, I, C> Div<C> for &'a Polynomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn div(self, rhs: C) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<I, C> DivAssign<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
    fn div_assign(&mut self, rhs: C){
        self.monomials = ((self as &Polynomial<I>) / rhs).monomials;
    }
}

impl<'a, I> CheckedDiv<&'a Monomial<I>> for Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn checked_div(&self, rhs: &'a Monomial<I>) -> Option<Self::Output> {
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

impl<'a, 'b, I> Div<&'a Monomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn div(self, rhs: &'a Monomial<I>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I> DivAssign<&'a Monomial<I>> for Polynomial<I> where I: Id {
    fn div_assign(&mut self, rhs: &'a Monomial<I>){
        self.monomials = ((self as &Polynomial<I>) / rhs).monomials;
    }
}

impl<'a, I> CheckedDiv<&'a Polynomial<I>> for Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn checked_div(&self, rhs: &'a Polynomial<I>) -> Option<Self::Output> {
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

impl<'a, 'b, I> Div<&'a Polynomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn div(self, rhs: &'a Polynomial<I>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I> DivAssign<&'a Polynomial<I>> for Polynomial<I> where I: Id {
    fn div_assign(&mut self, rhs: &'a Polynomial<I>){
        self.monomials = ((self as &Polynomial<I>) / rhs).monomials;
    }
}

impl<'a, I> Neg for &'a Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn neg(self) -> Self::Output {
        Polynomial{monomials: self.monomials.iter().cloned().map(|ref x| -x).collect()}
    }
}

impl<I, C> AddAssign<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
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

impl<'a, I, C> Add<C> for &'a Polynomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn add(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I> AddAssign<&'a Monomial<I>> for Polynomial<I> where I: Id {
    fn add_assign(&mut self, rhs: &'a Monomial<I>) {
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

impl<'a, 'b, I> Add<&'a Monomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn add(self, rhs: &'a Monomial<I>) -> Self::Output{
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, I> AddAssign<&'a Polynomial<I>> for Polynomial<I> where I: Id {
    fn add_assign(&mut self, rhs: &'a Polynomial<I>) {
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

impl<'a, 'b, I> Add<&'a Polynomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn add(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<I, C> SubAssign<C> for Polynomial<I> where I: Id, C: Clone + Into<i64> {
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

impl<'a, I, C> Sub<C> for &'a Polynomial<I> where I: Id, C: Clone + Into<i64> {
    type Output = Polynomial<I>;
    fn sub(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I> SubAssign<&'a Monomial<I>> for Polynomial<I> where I: Id  {
    fn sub_assign(&mut self, rhs: &'a Monomial<I>) {
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

impl<'a, 'b, I> Sub<&'a Monomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'a Monomial<I>) -> Self::Output{
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, I> SubAssign<&'a Polynomial<I>> for Polynomial<I> where I: Id {
    fn sub_assign(&mut self, rhs: &'a Polynomial<I>) {
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

impl<'a, 'b, I> Sub<&'a Polynomial<I>> for &'b Polynomial<I> where I: Id {
    type Output = Polynomial<I>;
    fn sub(self, rhs: &'a Polynomial<I>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}