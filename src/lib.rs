use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::convert::From;
use std::iter;


#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial {
    pub coefficient : i64,
    pub powers : Vec<(u16, u8)>
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial {
    pub monomials: Vec<Monomial>
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Registry{
    pub id: u16
}

pub trait IsConstant {
    fn is_constant(&self) -> bool;
}

impl IsConstant for Monomial {
    fn is_constant(&self) -> bool {
        self.powers.len() == 0
    }
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

impl Registry {
    pub fn specific_monomial_variable(&mut self, id: u16) -> Monomial {
        Monomial{coefficient: 1, powers: vec![(id, 1)]}
    }

    pub fn new_monomial_variable(&mut self) -> Monomial {
        let id = self.id;
        self.id += 1;
        self.specific_monomial_variable(id)
    }

    pub fn specific_variable(&mut self, id: u16) -> Polynomial {
        Polynomial{monomials: vec![self.specific_monomial_variable(id)]}
    }

    pub fn new_variable(&mut self) -> Polynomial {
        Polynomial{monomials: vec![self.new_monomial_variable()]}
    }

    pub fn reset(&mut self) {
        self.id = 0;
    }
}

impl<C> From<C> for Monomial where C: Clone + Into<i64>{
    fn from(c: C) -> Self{
        Monomial{coefficient: c.into(), powers: Vec::new()}
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

//impl From<Monomial> for Polynomial{
//    fn from(m: Monomial) -> Self{
//        Polynomial{monomials: vec![m]}
//    }
//}

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

impl Monomial{
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

impl<'a, 'b> Mul<&'a Polynomial> for &'b Monomial{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
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

pub trait CheckedDiv<RHS = Self> {
    type Output;
    fn checked_div(&self, rhs: RHS) -> Option<Self::Output>;
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

impl<'a> Neg for &'a Monomial {
    type Output = Monomial;
    fn neg(self) -> Self::Output {
        Monomial{coefficient: - self.coefficient, powers: self.powers.clone()}
    }
}

impl<'a> Neg for &'a Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        Polynomial{monomials: self.monomials.iter().cloned().map(|ref x| -x).collect()}
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

impl<'a, 'b> Add<&'a Polynomial> for &'b Monomial {
    type Output = Polynomial;
    fn add(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
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

impl<'a, 'b> Sub<&'a Polynomial> for &'b Monomial {
    type Output = Polynomial;
    fn sub(self, rhs: &'a Polynomial) -> Self::Output {
        let mut result = -rhs;
        result += self;
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

//macro_rules! single_impl {
//    ($trait_name: ident,  $method_name:ident , $sym_in: ty, $sym_out: ty, $type_name:ty) => {
//        impl<'a> $trait_name<&'a $sym_in> for $type_name{
//           type Output = $sym_out;
//           fn $method_name (self, rhs: &'a $sym_in) -> Self::Output {
//                rhs.$method_name (self)
//           }
//        }
//    };
//}
//
//macro_rules! auto_impl {
//    ($trait_name: ident,  $method_name:ident , $sym_in: ty, $sym_out: ty) => {
//        single_impl!($trait_name, $method_name, $sym_in, $sym_out, i64);
//        single_impl!($trait_name, $method_name, $sym_in, $sym_out, i32);
//        single_impl!($trait_name, $method_name, $sym_in, $sym_out, i16);
//        single_impl!($trait_name, $method_name, $sym_in, $sym_out, i8);
//    };
//    ($trait_name: ident,  $method_name:ident , $sym: ty) => {
//        single_impl!($trait_name, $method_name, $sym, $sym, i64);
//        single_impl!($trait_name, $method_name, $sym, $sym, i32);
//        single_impl!($trait_name, $method_name, $sym, $sym, i16);
//        single_impl!($trait_name, $method_name, $sym, $sym, i8);
//    };
//}
//
//
//single_impl!(Add, add, Monomial, Polynomial, i64);
//single_impl!(Add, add, Polynomial, Polynomial, i64);
//single_impl!(Mul, mul, Monomial, Monomial, i64);
//single_impl!(Mul, mul, Polynomial, Polynomial, i64);

// Implementations for i64

impl PartialEq<Monomial> for i64 {
    fn eq(&self, other: &Monomial) -> bool {
        other.eq(self)
    }
}

impl PartialEq<Polynomial> for i64 {
    fn eq(&self, other: &Polynomial) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Monomial> for i64 {
    fn partial_cmp(&self, other: &Monomial) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl PartialOrd<Polynomial> for i64 {
    fn partial_cmp(&self, other: &Polynomial) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None
        }
    }
}

impl<'a> Div<&'a Monomial> for i64{
    type Output = Option<Monomial>;
    fn div(self, rhs: &'a Monomial) -> Self::Output {
        if rhs.is_constant() {
            match self.checked_rem(rhs.coefficient) {
                Some(0) => Some(Monomial::from(self / rhs.coefficient)),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'a> Div<&'a Polynomial> for i64{
    type Output = Option<Polynomial>;
    fn div(self, rhs: &'a Polynomial) -> Self::Output {
        match rhs.monomials.len() {
            1 => match self / &(rhs.monomials[0]) {
                Some(m) => Some(Polynomial{monomials: vec![m]}),
                None => None
            },
            _ => None
        }
    }
}

impl<'a> Mul<&'a Monomial> for i64{
    type Output = Monomial;
    fn mul(self, rhs: &'a Monomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Mul<&'a Polynomial> for i64{
    type Output = Polynomial;
    fn mul(self, rhs: &'a Polynomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Add<&'a Monomial> for i64{
    type Output = Polynomial;
    fn add(self, rhs: &'a Monomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Add<&'a Polynomial> for i32{
    type Output = Polynomial;
    fn add(self, rhs: &'a Polynomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Sub<&'a Monomial> for i64{
    type Output = Polynomial;
    fn sub(self, rhs: &'a Monomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}

impl<'a> Sub<&'a Polynomial> for i64{
    type Output = Polynomial;
    fn sub(self, rhs: &'a Polynomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}