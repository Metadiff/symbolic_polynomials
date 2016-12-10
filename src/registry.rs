use std::collections::HashMap;

use traits::*;
use monomial::Monomial;
use polynomial::Polynomial;

/// A registry that keeps track of all the used `ids` for symbolic variables so far.
/// Additionally, it stores which ones correspond to a floor, ceil, min or max expression
/// which is used for evaluation.
#[derive(Clone, Default)]
#[repr(C)]
pub struct Registry{
    id: u16,
    floor_registry: HashMap<u16, (Polynomial, Polynomial)>,
    ceil_registry: HashMap<u16, (Polynomial, Polynomial)>,
    min_registry: HashMap<u16, (Polynomial, Polynomial)>,
    max_registry: HashMap<u16, (Polynomial, Polynomial)>
}

impl Registry {
    /// Returns a symbolic [`Monomial`] with the `id` provided to the power one.
    pub fn specific_monomial_variable(&mut self, id: u16) -> Monomial {
        Monomial{coefficient: 1, powers: vec![(id, 1)]}
    }

    /// Returns a symbolic [`Monomial`] with the lowest `id` not used so far.
    pub fn new_monomial_variable(&mut self) -> Monomial {
        let id = self.id;
        self.id += 1;
        self.specific_monomial_variable(id)
    }

    /// Returns a symbolic [`Polynomial`] with the `id` provided to the power one.
    pub fn specific_variable(&mut self, id: u16) -> Polynomial {
        Polynomial{monomials: vec![self.specific_monomial_variable(id)]}
    }

    /// Returns a symbolic [`Polynomial`] with the lowest `id` not used so far.
    pub fn new_variable(&mut self) -> Polynomial {
        Polynomial{monomials: vec![self.new_monomial_variable()]}
    }

    /// Resets the registry.
    pub fn reset(&mut self) {
        self.id = 0;
    }

    pub fn eval_monomial(&self, monomial: &Monomial) -> i64 {
        0
    }

    pub fn eval(&self, polynomial: &Polynomial) -> i64 {
        polynomial.monomials.iter().fold(0, |sum, val| sum + self.eval_monomial(val))
    }
}

macro_rules! impl_all {
    ($trait_name: ident,  $method_name:ident) => {
        impl<'a, C> $trait_name<C, &'a Monomial> for Registry where C: Clone + Into<i64>{
            type Output = Polynomial;
            fn $method_name(&mut self, left: C, right: &'a Monomial) -> Self::Output {
                self.$method_name(Polynomial::from(left), Polynomial::from(right))
            }
        }

        impl<'a, C> $trait_name<&'a Monomial, C> for Registry where C: Clone + Into<i64>{
            type Output = Polynomial;
            fn $method_name(&mut self, left: &'a Monomial, right: C) -> Self::Output {
                self.$method_name(Polynomial::from(left), Polynomial::from(right))
            }
        }

        impl<'a, 'b> $trait_name<&'a Monomial, &'b Monomial> for Registry {
            type Output = Polynomial;
            fn $method_name(&mut self, left: &'a Monomial, right: &'b Monomial) -> Self::Output {
                self.$method_name(Polynomial::from(left), Polynomial::from(right))
            }
        }

        impl $trait_name<Monomial, Monomial> for Registry {
            type Output = Polynomial;
            fn $method_name(&mut self, left: Monomial, right: Monomial) -> Self::Output {
                self.$method_name(Polynomial::from(left), Polynomial::from(right))
            }
        }

        impl<'a, C> $trait_name<C, &'a Polynomial> for Registry where C: Clone + Into<i64>{
            type Output = Polynomial;
            fn $method_name(&mut self, left: C, right: &'a Polynomial) -> Self::Output {
                self.$method_name(&Polynomial::from(left), right)
            }
        }

        impl<'a, C> $trait_name<&'a Polynomial, C> for Registry where C: Clone + Into<i64>{
            type Output = Polynomial;
            fn $method_name(&mut self, left: &'a Polynomial, right: C) -> Self::Output {
                self.$method_name(left, &Polynomial::from(right))
            }
        }
    };
}

impl<C, D> Floor<C, D> for Registry where C: Clone + Into<i64>, D: Clone + Into<i64>{
    type Output = i64;
    fn floor(&mut self, left: C, right: D) -> Self::Output {
        (left.into() as f64 / right.into() as f64).floor() as i64
    }
}

impl<'a, 'b> Floor<&'a Polynomial, &'b Polynomial> for Registry {
    type Output = Polynomial;
    fn floor(&mut self, left: &'a Polynomial, right: &'b Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(left);
            let v2 = self.eval(right);
            Polynomial::from(self.floor(v1, v2))
        } else {
            self.floor_registry.insert(self.id, (left.clone(), right.clone()));
            self.new_variable()
        }
    }
}

impl Floor<Polynomial, Polynomial> for Registry {
    type Output = Polynomial;
    fn floor(&mut self, left: Polynomial, right: Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(&left);
            let v2 = self.eval(&right);
            Polynomial::from(self.floor(v1, v2))
        } else {
            self.floor_registry.insert(self.id, (left, right));
            self.new_variable()
        }
    }
}

impl_all!(Floor, floor);

impl<C, D> Ceil<C, D> for Registry where C: Clone + Into<i64>, D: Clone + Into<i64>{
    type Output = i64;
    fn ceil(&mut self, left: C, right: D) -> Self::Output {
        (left.into() as f64 / right.into() as f64).ceil() as i64
    }
}

impl<'a, 'b> Ceil<&'a Polynomial, &'b Polynomial> for Registry {
    type Output = Polynomial;
    fn ceil(&mut self, left: &'a Polynomial, right: &'b Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(left);
            let v2 = self.eval(right);
            Polynomial::from(self.ceil(v1, v2))
        } else {
            self.ceil_registry.insert(self.id, (left.clone(), right.clone()));
            self.new_variable()
        }
    }
}

impl Ceil<Polynomial, Polynomial> for Registry {
    type Output = Polynomial;
    fn ceil(&mut self, left: Polynomial, right: Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(&left);
            let v2 = self.eval(&right);
            Polynomial::from(self.ceil(v1, v2))
        } else {
            self.ceil_registry.insert(self.id, (left, right));
            self.new_variable()
        }
    }
}

impl_all!(Ceil, ceil);

impl<C, D> Min<C, D> for Registry where C: Clone + Into<i64>, D: Clone + Into<i64>{
    type Output = i64;
    fn min(&mut self, left: C, right: D) -> Self::Output {
        let v1 = left.into();
        let v2 = right.into();
        if v1 < v2 {
            v1
        } else {
            v2
        }
    }
}

impl<'a, 'b> Min<&'a Polynomial, &'b Polynomial> for Registry {
    type Output = Polynomial;
    fn min(&mut self, left: &'a Polynomial, right: &'b Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(left);
            let v2 = self.eval(right);
            Polynomial::from(self.min(v1, v2))
        } else {
            self.min_registry.insert(self.id, (left.clone(), right.clone()));
            self.new_variable()
        }
    }
}

impl Min<Polynomial, Polynomial> for Registry {
    type Output = Polynomial;
    fn min(&mut self, left: Polynomial, right: Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(&left);
            let v2 = self.eval(&right);
            Polynomial::from(self.min(v1, v2))
        } else {
            self.min_registry.insert(self.id, (left, right));
            self.new_variable()
        }
    }
}

impl_all!(Min, min);

impl<C, D> Max<C, D> for Registry where C: Clone + Into<i64>, D: Clone + Into<i64>{
    type Output = i64;
    fn max(&mut self, left: C, right: D) -> Self::Output {
        let v1 = left.into();
        let v2 = right.into();
        if v1 > v2 {
            v1
        } else {
            v2
        }
    }
}

impl<'a, 'b> Max<&'a Polynomial, &'b Polynomial> for Registry {
    type Output = Polynomial;
    fn max(&mut self, left: &'a Polynomial, right: &'b Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(left);
            let v2 = self.eval(right);
            Polynomial::from(self.max(v1, v2))
        } else {
            self.max_registry.insert(self.id, (left.clone(), right.clone()));
            self.new_variable()
        }
    }
}

impl Max<Polynomial, Polynomial> for Registry {
    type Output = Polynomial;
    fn max(&mut self, left: Polynomial, right: Polynomial) -> Self::Output {
        if left.is_constant() && right.is_constant() {
            let v1 = self.eval(&left);
            let v2 = self.eval(&right);
            Polynomial::from(self.max(v1, v2))
        } else {
            self.max_registry.insert(self.id, (left, right));
            self.new_variable()
        }
    }
}

impl_all!(Max, max);
