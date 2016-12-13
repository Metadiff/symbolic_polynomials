use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use num::{Integer, One, Zero, Unsigned};

use functions::*;

pub trait Id: Clone + Ord + ::std::hash::Hash + VariableDisplay {}

impl<T> Id for T where T: Clone + Ord + ::std::hash::Hash + VariableDisplay {}

pub trait Power: Integer + One + Zero + Unsigned + Into<usize> + Clone + Ord +
::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Power for T where T:
    Integer + One + Zero + Unsigned + Into<usize> + Clone + Ord +
    ::std::fmt::Display + ::std::fmt::Debug {}

pub trait Coefficient: Integer + One +
    AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + Neg<Output=Self> +
    Clone + ::std::fmt::Display + ::std::fmt::Debug {}

impl<T> Coefficient for T where T: Integer + One +
    AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + Neg<Output=T> +
    Clone + ::std::fmt::Display + ::std::fmt::Debug {}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Monomial<I, C, P> where I: Id, C: Coefficient, P: Power {
    pub coefficient : C,
    pub powers : Vec<(Composite<I, C, P>, P)>
}

#[derive(Clone, Default, Eq)]
#[repr(C)]
pub struct Polynomial<I, C, P> where I: Id, C: Coefficient, P: Power {
    pub monomials: Vec<Monomial<I, C, P>>
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub enum Composite<I, C, P> where I: Id, C: Coefficient, P: Power {
    Variable(I),
    Floor(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Ceil(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Min(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>),
    Max(::std::rc::Rc<Polynomial<I, C, P>>, ::std::rc::Rc<Polynomial<I, C, P>>)
}
