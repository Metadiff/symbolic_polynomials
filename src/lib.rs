#[allow(unused_imports)]
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::fmt;
use std::cmp::{min, Ord, Ordering};
use std::convert::From;
use std::iter;

pub trait UpToCoefficient<Rhs> {
    fn up_to_coefficient(&self, other: Rhs) -> bool;
}

#[derive(Clone, Default, Eq)]
pub struct SymMonomial {
    pub coefficient : i64,
    pub powers : Vec<(u16, u8)>
}

#[derive(Clone, Default, Eq)]
pub struct SymPolynomial {
    pub monomials: Vec<SymMonomial>
}


impl SymMonomial{
    pub fn is_constant(&self) -> bool {
        self.powers.len() == 0
    }

    #[allow(dead_code)]
    pub fn variable(id: u16) -> SymMonomial {
        SymMonomial{coefficient: 1, powers: vec![(id, 1)]}
    }

    #[allow(dead_code)]
    pub fn eval() -> i64 {
        0
    }
}

impl SymPolynomial {
    pub fn is_constant(&self) -> bool {
        match self.monomials.len(){
            0 => true,
            1 => self.monomials[0].is_constant(),
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn variable(id: u16) -> SymPolynomial {
        SymPolynomial{monomials: vec![SymMonomial::variable(id)]}
    }

    #[allow(dead_code)]
    pub fn eval() -> i64 {
        0
    }
}

impl<T> From<T> for SymMonomial where i64: From<T>{
    fn from(t: T) -> Self{
        SymMonomial{coefficient: i64::from(t), powers: Vec::new()}
    }
}

impl<T> From<T> for SymPolynomial where i64: From<T>{
    fn from(t: T) -> Self{
        SymPolynomial{monomials: vec![SymMonomial::from(t)]}
    }
}

impl<'a> From<&'a SymMonomial> for SymPolynomial{
    fn from(t: &'a SymMonomial) -> Self{
        SymPolynomial{monomials: vec![t.clone()]}
    }
}

impl fmt::Display for SymMonomial {
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

impl fmt::Display for SymPolynomial {
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

impl fmt::Debug for SymMonomial {
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

impl fmt::Debug for SymPolynomial {
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

impl<'a> UpToCoefficient<&'a SymMonomial> for SymMonomial{
    fn up_to_coefficient(&self, other: &'a SymMonomial) -> bool {
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

impl<T> UpToCoefficient<T> for SymMonomial where i64: PartialEq<T> {
    fn up_to_coefficient(&self, _: T) -> bool {
        self.is_constant()
    }
}

impl PartialEq for SymMonomial{
    fn eq(&self, other: &SymMonomial) -> bool {
        match self.coefficient == other.coefficient {
            true => self.up_to_coefficient(other),
            false => false
        }
    }
}

impl PartialEq for SymPolynomial{
    fn eq(&self, other: &SymPolynomial) -> bool {
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

impl PartialOrd for SymMonomial {
    fn partial_cmp(&self, other: &SymMonomial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for SymPolynomial {
    fn partial_cmp(&self, other: &SymPolynomial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SymMonomial {
    fn cmp(&self, other: &SymMonomial) -> Ordering {
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

impl Ord for SymPolynomial {
    fn cmp(&self, other: &SymPolynomial) -> Ordering {
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

impl<C> MulAssign<C> for SymMonomial where i64: MulAssign<C> {
    fn mul_assign(&mut self, rhs: C){
        self.coefficient *= rhs;
    }
}

impl<'a, C> Mul<C> for &'a SymMonomial where i64: MulAssign<C> {
    type Output = SymMonomial;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<C> MulAssign<C> for SymPolynomial where i64: MulAssign<C>, C: Clone {
    fn mul_assign(&mut self, rhs: C){
        for m in self.monomials.iter_mut() {
            *m *= rhs.clone();
        }
    }
}

impl<'a, C> Mul<C> for &'a SymPolynomial where i64: MulAssign<C>, C: Clone {
    type Output = SymPolynomial;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a> MulAssign<&'a SymMonomial> for SymMonomial {
    fn mul_assign(&mut self, rhs: &'a SymMonomial){
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

impl<'a, 'b> Mul<&'a SymMonomial> for &'b SymMonomial{
    type Output = SymMonomial;
    fn mul(self, rhs: &'a SymMonomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a> MulAssign<&'a SymMonomial> for SymPolynomial {
    fn mul_assign(&mut self, rhs: &'a SymMonomial){
        for mut m in self.monomials.iter_mut() {
            *m *= rhs;
        }
    }
}

impl<'a, 'b> Mul<&'a SymMonomial> for &'b SymPolynomial{
    type Output = SymPolynomial;
    fn mul(self, rhs: &'a SymMonomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, 'b> Mul<&'a SymPolynomial> for &'b SymMonomial{
    type Output = SymPolynomial;
    fn mul(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
        result
    }
}

impl<'a> MulAssign<&'a SymPolynomial> for SymPolynomial {
    fn mul_assign(&mut self, rhs: &'a SymPolynomial){
        match self.monomials.len() {
            0 => {},
            _ => {
                let mut result = SymPolynomial::default();
                for m in self.monomials.iter(){
                    result += &(m * rhs);
                }
                self.monomials = result.monomials;
            }
        }
    }
}

impl<'a, 'b> Mul<&'a SymPolynomial> for &'b SymPolynomial{
    type Output = SymPolynomial;
    fn mul(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, C> Div<C> for &'a SymMonomial where i64: From<C>, C: Clone {
    type Output = Option<SymMonomial>;
    fn div(self, rhs: C) -> Self::Output {
        match self.coefficient.checked_rem(i64::from(rhs.clone())) {
            Some(0) => Some(SymMonomial {
                coefficient: self.coefficient / i64::from(rhs),
                powers: self.powers.clone()
            }),
            _ => None
        }
    }
}

impl<C> DivAssign<C> for SymMonomial where i64: From<C> + DivAssign<C>, C: Clone {
    fn div_assign(&mut self, rhs: C){
        match self.coefficient.checked_rem(i64::from(rhs.clone())) {
            Some(0) => {self.coefficient /= rhs},
            _ => panic!("Non integer division via DivAssign")
        }
    }
}


impl<'a, C> Div<C> for &'a SymPolynomial where i64: From<C>, C: Clone {
    type Output = Option<SymPolynomial>;
    fn div(self, rhs: C) -> Self::Output {
        let result = SymPolynomial{monomials: self.monomials.iter()
            .cloned()
            .filter_map(|ref m| m / rhs.clone())
            .collect()};
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<C> DivAssign<C> for SymPolynomial where i64: From<C>, C: Clone {
    fn div_assign(&mut self, rhs: C){
        match (self as &SymPolynomial) / rhs {
            Some(r) => {self.monomials = r.monomials},
            None => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a, 'b> Div<&'a SymMonomial> for &'b SymMonomial {
    type Output = Option<SymMonomial>;
    fn div(self, rhs: &'a SymMonomial) -> Self::Output {
        match self.coefficient.checked_rem(rhs.coefficient.clone()) {
            Some(0) => {
                let mut result = SymMonomial{
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

impl<'a> DivAssign<&'a SymMonomial> for SymMonomial {
    fn div_assign(&mut self, rhs: &'a SymMonomial){
        match (self as &SymMonomial) / rhs {
            Some(r) => {self.coefficient = r.coefficient; self.powers = r.powers.clone()},
            None => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a, 'b> Div<&'a SymMonomial> for &'b SymPolynomial {
    type Output = Option<SymPolynomial>;
    fn div(self, rhs: &'a SymMonomial) -> Self::Output {
        let result = SymPolynomial{monomials: self.monomials.iter()
            .cloned()
            .filter_map(|ref m| m / rhs)
            .collect()};
        if result.monomials.len() != self.monomials.len() {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a> DivAssign<&'a SymMonomial> for SymPolynomial {
    fn div_assign(&mut self, rhs: &'a SymMonomial){
        match (self as &SymPolynomial) / rhs {
            Some(r) => {self.monomials = r.monomials;},
            None => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a, 'b> Div<&'a SymPolynomial> for &'b SymPolynomial {
    type Output = Option<SymPolynomial>;
    fn div(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = SymPolynomial::default();
        let mut reminder = self.clone();
        while ! reminder.is_constant() {
            match &(reminder.monomials[0]) / &(rhs.monomials[0]) {
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

impl<'a> DivAssign<&'a SymPolynomial> for SymPolynomial {
    fn div_assign(&mut self, rhs: &'a SymPolynomial){
        match (self as &SymPolynomial) / rhs {
            Some(r) => {self.monomials = r.monomials;},
            None => panic!("Non integer division via DivAssign")
        }
    }
}

impl<'a> Neg for &'a SymMonomial {
    type Output = SymMonomial;
    fn neg(self) -> Self::Output {
        SymMonomial{coefficient: - self.coefficient, powers: self.powers.clone()}
    }
}

impl<'a> Neg for &'a SymPolynomial {
    type Output = SymPolynomial;
    fn neg(self) -> Self::Output {
        SymPolynomial{monomials: self.monomials.iter().cloned().map(|ref x| -x).collect()}
    }
}

impl<'a, C> Add<C> for &'a SymMonomial where i64: AddAssign<C> + From<C>, C: PartialEq<i64> {
    type Output = SymPolynomial;
    fn add(self, rhs: C) -> Self::Output {
        if rhs == 0 {
            SymPolynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == -self.coefficient {
                SymPolynomial::default()
            } else {
                let mut result = SymPolynomial::from(self);
                result.monomials[0].coefficient += rhs;
                result
            }
        } else {
            SymPolynomial{monomials: vec![self.clone(), SymMonomial::from(rhs)]}
        }
    }
}

impl<C> AddAssign<C> for SymPolynomial where i64: AddAssign<C> + From<C>, C: Clone + PartialEq<i64> {
    fn add_assign(&mut self, rhs: C) {
        if rhs != 0 {
            let mut remove: bool = false;
            match self.monomials.last_mut() {
                Some(ref mut l) => {
                    if l.is_constant() {
                        l.coefficient += rhs.clone();
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
                self.monomials.push(SymMonomial::from(rhs));
            }
        }
    }
}

impl<'a, C> Add<C> for &'a SymPolynomial where i64: AddAssign<C> + From<C>, C: Clone + PartialEq<i64> {
    type Output = SymPolynomial;
    fn add(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, 'b> Add<&'b SymMonomial> for &'a SymMonomial{
    type Output = SymPolynomial;
    fn add(self, rhs: &'b SymMonomial) -> Self::Output {
        if rhs.coefficient == 0 && self.coefficient == 0 {
            SymPolynomial::default()
        } else if rhs.coefficient == 0 {
            SymPolynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            SymPolynomial{monomials: vec![rhs.clone()]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == -rhs.coefficient {
                SymPolynomial::default()
            } else {
                let mut result = SymPolynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient += rhs.coefficient;
                result
            }
        } else if self > rhs {
            SymPolynomial{monomials: vec![self.clone(), rhs.clone()]}
        } else {
            SymPolynomial{monomials: vec![rhs.clone(), self.clone()]}
        }
    }
}

impl<'a> AddAssign<&'a SymMonomial> for SymPolynomial {
    fn add_assign(&mut self, rhs: &'a SymMonomial) {
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

impl<'a, 'b> Add<&'a SymMonomial> for &'b SymPolynomial {
    type Output = SymPolynomial;
    fn add(self, rhs: &'a SymMonomial) -> Self::Output{
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<'a, 'b> Add<&'a SymPolynomial> for &'b SymMonomial {
    type Output = SymPolynomial;
    fn add(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
        result
    }
}

impl<'a> AddAssign<&'a SymPolynomial> for SymPolynomial {
    fn add_assign(&mut self, rhs: &'a SymPolynomial) {
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

impl<'a, 'b> Add<&'a SymPolynomial> for &'b SymPolynomial {
    type Output = SymPolynomial;
    fn add(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}


impl<'a, C> Sub<C> for &'a SymMonomial where i64: SubAssign<C> + From<C>, C: PartialEq<i64> {
    type Output = SymPolynomial;
    fn sub(self, rhs: C) -> Self::Output {
        if rhs == 0 {
            SymPolynomial{monomials: vec![self.clone()]}
        } else if self.is_constant(){
            if rhs == self.coefficient {
                SymPolynomial::default()
            } else {
                let mut result = SymPolynomial::from(self);
                result.monomials[0].coefficient -= rhs;
                result
            }
        } else {
            SymPolynomial{monomials: vec![self.clone(), SymMonomial::from(-i64::from(rhs))]}
        }
    }
}

impl<C> SubAssign<C> for SymPolynomial where i64: SubAssign<C> + From<C>, C: PartialEq<i64> {
    fn sub_assign(&mut self, rhs: C) {
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
                self.monomials.push(SymMonomial::from(-i64::from(rhs)));
            }
        }
    }
}

impl<'a, C> Sub<C> for &'a SymPolynomial where i64: SubAssign<C> + From<C>, C: PartialEq<i64> {
    type Output = SymPolynomial;
    fn sub(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, 'b> Sub<&'b SymMonomial> for &'a SymMonomial{
    type Output = SymPolynomial;
    fn sub(self, rhs: &'b SymMonomial) -> Self::Output {
        if self.coefficient == 0 && rhs.coefficient == 0 {
            SymPolynomial::default()
        } else if rhs.coefficient == 0 {
            SymPolynomial{monomials: vec![self.clone()]}
        } else if self.coefficient == 0 {
            SymPolynomial{monomials: vec![-rhs]}
        } else if self.up_to_coefficient(rhs){
            if self.coefficient == rhs.coefficient {
                SymPolynomial::default()
            } else {
                let mut result = SymPolynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient -= rhs.coefficient;
                result
            }
        } else if self > rhs {
            SymPolynomial{monomials: vec![self.clone(), -rhs]}
        } else {
            SymPolynomial{monomials: vec![-rhs, self.clone()]}
        }
    }
}

impl<'a> SubAssign<&'a SymMonomial> for SymPolynomial {
    fn sub_assign(&mut self, rhs: &'a SymMonomial) {
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

impl<'a, 'b> Sub<&'a SymMonomial> for &'b SymPolynomial {
    type Output = SymPolynomial;
    fn sub(self, rhs: &'a SymMonomial) -> Self::Output{
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a, 'b> Sub<&'a SymPolynomial> for &'b SymMonomial {
    type Output = SymPolynomial;
    fn sub(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = -rhs;
        result += self;
        result
    }
}

impl<'a> SubAssign<&'a SymPolynomial> for SymPolynomial {
    fn sub_assign(&mut self, rhs: &'a SymPolynomial) {
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

impl<'a, 'b> Sub<&'a SymPolynomial> for &'b SymPolynomial {
    type Output = SymPolynomial;
    fn sub(self, rhs: &'a SymPolynomial) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<'a> Mul<&'a SymMonomial> for i64{
    type Output = SymMonomial;
    fn mul(self, rhs: &'a SymMonomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Mul<&'a SymPolynomial> for i64{
    type Output = SymPolynomial;
    fn mul(self, rhs: &'a SymPolynomial) -> Self::Output {
        rhs.mul(self)
    }
}

impl<'a> Div<&'a SymMonomial> for i64{
    type Output = Option<SymMonomial>;
    fn div(self, rhs: &'a SymMonomial) -> Self::Output {
        if rhs.is_constant() {
            match self.checked_rem(rhs.coefficient) {
                Some(0) => Some(SymMonomial::from(self / rhs.coefficient)),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'a> Div<&'a SymPolynomial> for i64{
    type Output = Option<SymPolynomial>;
    fn div(self, rhs: &'a SymPolynomial) -> Self::Output {
        match rhs.monomials.len() {
            1 => match self / &(rhs.monomials[0]) {
                Some(m) => Some(SymPolynomial{monomials: vec![m]}),
                None => None
            },
            _ => None
        }
    }
}


impl<'a> Add<&'a SymMonomial> for i64{
    type Output = SymPolynomial;
    fn add(self, rhs: &'a SymMonomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Add<&'a SymPolynomial> for i64{
    type Output = SymPolynomial;
    fn add(self, rhs: &'a SymPolynomial) -> Self::Output {
        rhs.add(self)
    }
}

impl<'a> Sub<&'a SymMonomial> for i64{
    type Output = SymPolynomial;
    fn sub(self, rhs: &'a SymMonomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}

impl<'a> Sub<&'a SymPolynomial> for i64{
    type Output = SymPolynomial;
    fn sub(self, rhs: &'a SymPolynomial) -> Self::Output {
        -&(rhs.add(-self))
    }
}