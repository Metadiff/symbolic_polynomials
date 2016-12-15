use std::rc::Rc;
use std::collections::HashMap;
extern crate symints;
use symints::*;

type TestMonomial = Monomial<String, i64, u8>;
type TestPolynomial = Polynomial<String, i64, u8>;

#[test]
pub fn max_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let thirteen_v2 = max(&thirteen, &three);
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = max(&a_square, &a);
    let max_a_square_b = max(&a_square, &b);
    let max_a_third_b = max(&a_third, &b);

    assert!(thirteen_v2 == 13 && 13 == thirteen_v2);
    assert!(a_v2.monomials.len() == 1);
    assert!(a_v2.monomials[0].coefficient == 1);
    assert!(a_v2.monomials[0].powers == vec![(Composite::Max(Rc::new(a_square.clone()), Rc::new(a)), 1)]);
    assert!(max_a_square_b.monomials.len() == 1);
    assert!(max_a_square_b.monomials[0].coefficient == 1);
    assert!(max_a_square_b.monomials[0].powers == vec![(Composite::Max(Rc::new(a_square), Rc::new(b.clone())), 1)]);
    assert!(max_a_third_b.monomials.len() == 1);
    assert!(max_a_third_b.monomials[0].coefficient == 1);
    assert!(max_a_third_b.monomials[0].powers == vec![(Composite::Max(Rc::new(a_third), Rc::new(b)), 1)]);
    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert!(thirteen.evaluate(&values) == Ok(13));
    assert!(max_a_square_b.evaluate(&values) == Ok(13));
    assert!(max_a_third_b.evaluate(&values) == Ok(27));
}

#[test]
pub fn min_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let three_v2 = min(&thirteen, &three);
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = min(&a_square, &a);
    let min_a_square_b = min(&a_square, &b);
    let min_a_third_b = min(&a_third, &b);

    assert!(three_v2 == 3 && 3 == three_v2);
    assert!(a_v2.monomials.len() == 1);
    assert!(a_v2.monomials[0].coefficient == 1);
    assert!(a_v2.monomials[0].powers == vec![(Composite::Min(Rc::new(a_square.clone()), Rc::new(a)), 1)]);
    assert!(min_a_square_b.monomials.len() == 1);
    assert!(min_a_square_b.monomials[0].coefficient == 1);
    assert!(min_a_square_b.monomials[0].powers == vec![(Composite::Min(Rc::new(a_square), Rc::new(b.clone())), 1)]);
    assert!(min_a_third_b.monomials.len() == 1);
    assert!(min_a_third_b.monomials[0].coefficient == 1);
    assert!(min_a_third_b.monomials[0].powers == vec![(Composite::Min(Rc::new(a_third), Rc::new(b)), 1)]);

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert!(three_v2.evaluate(&values) == Ok(3));
    assert!(min_a_square_b.evaluate(&values) == Ok(9));
    assert!(min_a_third_b.evaluate(&values) == Ok(13));
}

#[test]
pub fn ceil_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let five = ceil(&thirteen, &three);
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = ceil(&a_square, &a);
    let ceil_a_square_b = ceil(&a_square, &b);
    let ceil_a_third_b = ceil(&a_third, &b);

    assert!(five == 5 && 5 == five);
    assert!(a_v2 == a);
    assert!(ceil_a_square_b.monomials.len() == 1);
    assert!(ceil_a_square_b.monomials[0].coefficient == 1);
    assert!(ceil_a_square_b.monomials[0].powers == vec![(Composite::Ceil(Rc::new(a_square), Rc::new(b.clone())), 1)]);
    assert!(ceil_a_third_b.monomials.len() == 1);
    assert!(ceil_a_third_b.monomials[0].coefficient == 1);
    assert!(ceil_a_third_b.monomials[0].powers == vec![(Composite::Ceil(Rc::new(a_third), Rc::new(b)), 1)]);

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert!(five.evaluate(&values) == Ok(5));
    assert!(ceil_a_square_b.evaluate(&values) == Ok(1));
    assert!(ceil_a_third_b.evaluate(&values) == Ok(3));
}

#[test]
pub fn floor_test() {
    let thirteen = TestPolynomial::from(13);
    let three = TestPolynomial::from(3);
    let four = floor(&thirteen, &three);
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let a_square = &a * &a;
    let a_third = &a_square * &a;
    let a_v2 = floor(&a_square, &a);
    let floor_a_square_b = floor(&a_square, &b);
    let floor_a_third_b = floor(&a_third, &b);

    assert!(four == 4 && 4 == four);
    assert!(a_v2 == a && a == a_v2);
    assert!(floor_a_square_b.monomials.len() == 1);
    assert!(floor_a_square_b.monomials[0].coefficient == 1);
    assert!(floor_a_square_b.monomials[0].powers == vec![(Composite::Floor(Rc::new(a_square), Rc::new(b.clone())), 1)]);
    assert!(floor_a_third_b.monomials.len() == 1);
    assert!(floor_a_third_b.monomials[0].coefficient == 1);
    assert!(floor_a_third_b.monomials[0].powers == vec![(Composite::Floor(Rc::new(a_third), Rc::new(b)), 1)]);

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    assert!(four.evaluate(&values) == Ok(4));
    assert!(floor_a_square_b.evaluate(&values) == Ok(0));
    assert!(floor_a_third_b.evaluate(&values) == Ok(2));
}
