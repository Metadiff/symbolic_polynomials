use std::rc::Rc;
use std::collections::{HashSet, HashMap};
extern crate symbolic_polynomials;
extern crate num;
use symbolic_polynomials::*;
use num::Integer;

#[allow(dead_code)]
type TestMonomial = Monomial<String, i64, u8>;
#[allow(dead_code)]
type TestPolynomial = Polynomial<String, i64, u8>;

#[test]
pub fn max_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let thirteen_v2 = max(&thirteen, &three);
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = max(&a_square, &a);
    let max_a_square_b = max(&a_square, &b);
    let max_a_third_b = max(&a_third, &b);

    assert_eq!(thirteen_v2, 13);
    assert_eq!(13, thirteen_v2);
    assert_eq!(a_v2.monomials.len(), 1);
    assert_eq!(a_v2.monomials[0].coefficient, 1);
    assert_eq!(
        a_v2.monomials[0].powers,
        vec![(Composite::Max(Rc::new(a_square.clone()), Rc::new(a)), 1)]
    );
    assert_eq!(max_a_square_b.monomials.len(), 1);
    assert_eq!(max_a_square_b.monomials[0].coefficient, 1);
    assert_eq!(
        max_a_square_b.monomials[0].powers,
        vec![(Composite::Max(Rc::new(a_square), Rc::new(b.clone())), 1)]
    );
    assert_eq!(max_a_third_b.monomials.len(), 1);
    assert_eq!(max_a_third_b.monomials[0].coefficient, 1);
    assert_eq!(
        max_a_third_b.monomials[0].powers,
        vec![(Composite::Max(Rc::new(a_third), Rc::new(b)), 1)]
    );
    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert_eq!(thirteen.eval(&values), Ok(13));
    assert_eq!(max_a_square_b.eval(&values), Ok(13));
    assert_eq!(max_a_third_b.eval(&values), Ok(27));
}

#[test]
pub fn min_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let three_v2 = min(&thirteen, &three);
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = min(&a_square, &a);
    let min_a_square_b = min(&a_square, &b);
    let min_a_third_b = min(&a_third, &b);

    assert_eq!(three_v2, 3);
    assert_eq!(3, three_v2);
    assert_eq!(a_v2.monomials.len(), 1);
    assert_eq!(a_v2.monomials[0].coefficient, 1);
    assert_eq!(
        a_v2.monomials[0].powers,
        vec![(Composite::Min(Rc::new(a_square.clone()), Rc::new(a)), 1)]
    );
    assert_eq!(min_a_square_b.monomials.len(), 1);
    assert_eq!(min_a_square_b.monomials[0].coefficient, 1);
    assert_eq!(
        min_a_square_b.monomials[0].powers,
        vec![(Composite::Min(Rc::new(a_square), Rc::new(b.clone())), 1)]
    );
    assert_eq!(min_a_third_b.monomials.len(), 1);
    assert_eq!(min_a_third_b.monomials[0].coefficient, 1);
    assert_eq!(
        min_a_third_b.monomials[0].powers,
        vec![(Composite::Min(Rc::new(a_third), Rc::new(b)), 1)]
    );

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert_eq!(three_v2.eval(&values), Ok(3));
    assert_eq!(min_a_square_b.eval(&values), Ok(9));
    assert_eq!(min_a_third_b.eval(&values), Ok(13));
}

#[test]
pub fn ceil_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let five = ceil(&thirteen, &three);
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = ceil(&a_square, &a);
    let ceil_a_square_b = ceil(&a_square, &b);
    let ceil_a_third_b = ceil(&a_third, &b);

    assert_eq!(five, 5);
    assert_eq!(5, five);
    assert_eq!(a_v2, a);
    assert_eq!(ceil_a_square_b.monomials.len(), 1);
    assert_eq!(ceil_a_square_b.monomials[0].coefficient, 1);
    assert_eq!(
        ceil_a_square_b.monomials[0].powers,
        vec![(Composite::Ceil(Rc::new(a_square), Rc::new(b.clone())), 1)]
    );
    assert_eq!(ceil_a_third_b.monomials.len(), 1);
    assert_eq!(ceil_a_third_b.monomials[0].coefficient, 1);
    assert_eq!(
        ceil_a_third_b.monomials[0].powers,
        vec![(Composite::Ceil(Rc::new(a_third), Rc::new(b)), 1)]
    );

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert_eq!(five.eval(&values), Ok(5));
    assert_eq!(ceil_a_square_b.eval(&values), Ok(1));
    assert_eq!(ceil_a_third_b.eval(&values), Ok(3));
}

#[test]
pub fn floor_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let four = floor(&thirteen, &three);
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = floor(&a_square, &a);
    let floor_a_square_b = floor(&a_square, &b);
    let floor_a_third_b = floor(&a_third, &b);

    assert_eq!(four, 4);
    assert_eq!(4, four);
    assert_eq!(a_v2, a);
    assert_eq!(a, a_v2);
    assert_eq!(floor_a_square_b.monomials.len(), 1);
    assert_eq!(floor_a_square_b.monomials[0].coefficient, 1);
    assert_eq!(
        floor_a_square_b.monomials[0].powers,
        vec![(Composite::Floor(Rc::new(a_square), Rc::new(b.clone())), 1)]
    );
    assert_eq!(floor_a_third_b.monomials.len(), 1);
    assert_eq!(floor_a_third_b.monomials[0].coefficient, 1);
    assert_eq!(
        floor_a_third_b.monomials[0].powers,
        vec![(Composite::Floor(Rc::new(a_third), Rc::new(b)), 1)]
    );

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert_eq!(four.eval(&values), Ok(4));
    assert_eq!(floor_a_square_b.eval(&values), Ok(0));
    assert_eq!(floor_a_third_b.eval(&values), Ok(2));
}

#[test]
pub fn unique_identifiers_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let four = floor(&thirteen, &three);
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let floor_a_square_b = floor(&a_square, &b);
    let floor_a_third_b = floor(&a_third, &b);

    let mut set = HashSet::new();
    thirteen.unique_identifiers(&mut set);
    assert_eq!(set.len(), 0);
    three.unique_identifiers(&mut set);
    assert_eq!(set.len(), 0);
    four.unique_identifiers(&mut set);
    assert_eq!(set.len(), 0);

    a.unique_identifiers(&mut set);
    assert_eq!(set.len(), 1);
    a_square.unique_identifiers(&mut set);
    assert_eq!(set.len(), 1);
    a_third.unique_identifiers(&mut set);
    assert_eq!(set.len(), 1);

    b.unique_identifiers(&mut set);
    assert_eq!(set.len(), 2);
    floor_a_square_b.unique_identifiers(&mut set);
    assert_eq!(set.len(), 2);
    floor_a_third_b.unique_identifiers(&mut set);
    assert_eq!(set.len(), 2);

    set.clear();
    floor_a_square_b.unique_identifiers(&mut set);
    assert_eq!(set.len(), 2);

    set.clear();
    floor_a_third_b.unique_identifiers(&mut set);
    assert_eq!(set.len(), 2);
}

#[test]
pub fn deduce_values_test1() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 1;
    let b_val: i64 = 3;
    let c_val: i64 = 7;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // a
    let poly1 = a.clone();
    let val1 = a_val;
    implicit_values.push((poly1.clone(), val1));
    // 2ab + 1
    let poly2 = 2 * &a * &b + 1;
    let val2 = 2 * a_val * b_val + 1;
    implicit_values.push((poly2.clone(), val2));
    // 5a^2b^2c^2 + a^2b + 3
    let poly3 = 5 * &a * &a * &b * &b * &c * &c + &a * &a * &b + 3;
    let val3 = 5 * a_val * a_val * b_val * b_val * c_val * c_val + a_val * a_val * b_val + 3;
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test2() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 2;
    let b_val: i64 = 3;
    let c_val: i64 = 5;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // abc^2 + abc + 1
    let poly1 = &a * &b * &c * (&c + 1) + 1;
    let val1 = a_val * b_val * c_val * (c_val + 1) + 1;
    implicit_values.push((poly1.clone(), val1));
    // a^2 + c^2 + 2
    let poly2 = &a * &a + &c * &c + 2;
    let val2 = a_val * a_val + c_val * c_val + 2;
    implicit_values.push((poly2.clone(), val2));
    // 5c
    let poly3 = 5 * &c;
    let val3 = 5 * c_val;
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test3() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 1;
    let b_val: i64 = 2;
    let c_val: i64 = 3;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // 3b^2
    let poly1 = 3 * &b * &b;
    let val1 = 3 * b_val * b_val;
    implicit_values.push((poly1.clone(), val1));
    // a^3 + b^3 - 10
    let poly2 = &a * &a * &a + &b * &b * &b - 10;
    let val2 = a_val * a_val * a_val + b_val * b_val * b_val - 10;
    implicit_values.push((poly2.clone(), val2));
    // ab + ac + bc + 3
    let poly3 = &a * &b + &a * &c + &b * &c + 3;
    let val3 = a_val * b_val + a_val * c_val + b_val * c_val + 3;
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test_floor_min() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 1;
    let b_val: i64 = 3;
    let c_val: i64 = 7;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // a
    let poly1 = a.clone();
    let val1 = a_val;
    implicit_values.push((poly1.clone(), val1));
    // 2ab + 1
    let poly2 = 2 * &a * &b + 1;
    let val2 = 2 * a_val * b_val + 1;
    let two = TestPolynomial::from(2);
    implicit_values.push((poly2.clone(), val2));
    // 5a^2b^2c^2 + floor(ab^2, 2) + min(a^2, b^2) + 3
    let poly3 = 5 * &a * &a * &b * &b * &c * &c + floor(&a * &b * &b, &two) +
                min(&a * &a, &b * &b) + 3;
    let val3 = 5 * a_val * a_val * b_val * b_val * c_val * c_val +
               (a_val * b_val * b_val).div_floor(&2) +
               ::std::cmp::min(a_val * a_val, b_val * b_val) + 3;
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test_ceil_max() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 2;
    let b_val: i64 = 3;
    let c_val: i64 = 5;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // abc^2 + abc + 1
    let poly1 = &a * &b * &c * (&c + 1) + 1;
    let val1 = a_val * b_val * c_val + a_val * c_val * c_val * b_val + 1;
    let six = TestPolynomial::from(6);
    implicit_values.push((poly1.clone(), val1));
    // a^2 + ceil(c^2, 6) + max(c^2, 12) + 2
    let poly2 = &a * &a + ceil(&c * &c, &six) + max(&c * &c, TestPolynomial::from(12)) + 2;
    let mut val2 = a_val * a_val + (c_val * c_val).div_floor(&6) +
                   ::std::cmp::max(c_val * c_val, 12) + 2;
    if c_val * c_val % 6 != 0 {
        val2 += 1;
    }
    implicit_values.push((poly2.clone(), val2));
    // 5c
    let poly3 = 5 * &c;
    let val3 = 5 * c_val;
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test_all() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 2;
    let b_val: i64 = 3;
    let c_val: i64 = 5;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // 3b^2
    let poly1 = 3 * &b * &b;
    let val1 = 3 * b_val * b_val;
    implicit_values.push((poly1.clone(), val1));
    // a^3 + floor(b^3, 3) - 10 - min(b^2, 17)
    let poly2 = &a * &a * &a + floor(&b * &b * &b, TestPolynomial::from(3)) - 10 -
                min(&b * &b, TestPolynomial::from(17));
    let val2 = a_val * a_val * a_val + (b_val * b_val * b_val).div_floor(&3) - 10 -
               ::std::cmp::min(b_val * b_val, 17);
    implicit_values.push((poly2.clone(), val2));
    // ceil(7ab, 5) + ac + bc + 3 + max(ab - 5, a + 2b)
    let poly3 = ceil(7 * &a * &b, TestPolynomial::from(5)) + &a * &c + &b * &c + 3 +
                max(&a * &b - 5, &a + 2 * &b);
    let mut val3 = (7 * a_val * b_val).div_floor(&5) + a_val * c_val + b_val * c_val + 3 +
                   ::std::cmp::max(a_val * b_val - 5, a_val + 2 * b_val);
    if 7 * a_val * b_val % 5 != 0 {
        val3 += 1;
    }
    implicit_values.push((poly3.clone(), val3));
    let values = deduce_values(&implicit_values).unwrap();

    assert_eq!(a.eval(&values), Ok(a_val));
    assert_eq!(b.eval(&values), Ok(b_val));
    assert_eq!(c.eval(&values), Ok(c_val));

    assert_eq!(poly1.eval(&values), Ok(val1));
    assert_eq!(poly2.eval(&values), Ok(val2));
    assert_eq!(poly3.eval(&values), Ok(val3));
}

#[test]
pub fn deduce_values_test_fails() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let a_val: i64 = 1;
    let b_val: i64 = 2;
    let c_val: i64 = 3;
    let mut implicit_values = Vec::<(TestPolynomial, i64)>::new();

    // ab^2
    let poly1 = &a * &b * &b;
    let val1 = a_val * b_val * b_val;
    implicit_values.push((poly1.clone(), val1));
    // 2b + 1
    let poly2 = 2 * &b + 1;
    let val2 = 2 * b_val + 1;
    implicit_values.push((poly2.clone(), val2));
    // ac^2 + bc + 2
    let poly3 = &a * &c * &c + &b * &c + 2;
    let val3 = a_val * c_val * c_val + b_val * c_val + 2;
    implicit_values.push((poly3.clone(), val3));
    assert_eq!(deduce_values(&implicit_values), Err("Could not deduce all variables.".into()));

    // 2bc + 1
    let poly2 = 2 * &b * &c + 1;
    let val2 = 2 * b_val * c_val + 1;
    implicit_values.remove(1);
    implicit_values.push((poly2.clone(), val2));
    assert_eq!(deduce_values(&implicit_values), Err("Could not deduce all variables.".into()));
}
