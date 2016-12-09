//!
//!
//!
//!
//!

mod traits;
mod monomial;
mod polynomial;
mod registry;

pub use traits::*;
pub use monomial::Monomial;
pub use polynomial::Polynomial;
pub use registry::Registry;


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

