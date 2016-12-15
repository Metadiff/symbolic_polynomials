use std::collections::HashMap;
extern crate symints;
use symints::*;

type TestMonomial = Monomial<String, i64, u8>;
type TestPolynomial = Polynomial<String, i64, u8>;

#[test]
pub fn constructor() {
    let a: TestPolynomial = primitive("a".into());
    let b_mon = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let b = TestPolynomial::from(&(5 * &b_mon));
    let minus_six = TestPolynomial::from(-6);
    let thirteen = TestPolynomial::from(13);

    assert!(minus_six.is_constant());
    assert!(minus_six.monomials.len() == 1);
    assert!(minus_six.monomials[0].coefficient == -6);
    assert!(minus_six.monomials[0].powers.len() == 0);

    assert!(thirteen.is_constant());
    assert!(thirteen.monomials.len() == 1);
    assert!(thirteen.monomials[0].coefficient == 13);
    assert!(thirteen.monomials[0].powers.len() == 0);

    assert!(!a.is_constant());
    assert!(a.monomials.len() == 1);
    assert!(a.monomials[0].coefficient == 1);
    assert!(a.monomials[0].powers == vec![(Composite::Variable("a".into()), 1)]);

    assert!(!b.is_constant());
    assert!(b.monomials.len() == 1);
    assert!(b.monomials[0].coefficient == 5);
    assert!(b.monomials[0].powers == vec![(Composite::Variable("b".into()), 1)]);
}

#[test]
pub fn partial_eq_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let a_v2: TestPolynomial = primitive("a".into());
    let b_v2: TestPolynomial = primitive("b".into());
    // ab
    let ab = &a * &b;
    // a + b
    let a_plus_b = &a + &b;
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // a^2 + b^2 + 2ab = (a + b)^2
    let a_plus_b_square = &(&a_square + &b_square) + &(2 * &ab);

    assert!(a != 1 && 1 != a);
    assert!(b != 1 && 1 != b);

    assert!(a == a_v2 && a_v2 == a);
    assert!(b == b_v2 && b_v2 == b);

    assert!(ab != a && a != ab);
    assert!(ab != b && b != ab);

    assert!(a_square != a && a != a_square);
    assert!(b_square != b && b != b_square);
    assert!(a_plus_b_square == (&a_plus_b * &a_plus_b));
}

#[test]
pub fn ord_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // a^2b
    let a_square_b = &(&a * &a) * &b;
    // a^2b + a
    let a_square_b_plus_a = &a_square_b + &a;
    // a^2b + b
    let a_square_b_plus_b = &a_square_b + &b;
    // a + b
    let a_plus_b = &a + &b;
    // (a + b)^2
    let a_plus_b_square = &a_plus_b * &a_plus_b;

    assert!(a > 2 && 2 < a);
    assert!(b > 2 && 2 < b);

    assert!(a > b_square && b_square < a);
    assert!(b < b_square && b_square > b);
    assert!(a < a_square && a_square > a);
    assert!(b < a_square && a_square > b);
    assert!(b_square < a_square && a_square > b_square);

    assert!(a_square_b > a_square && a_square < a_square_b);
    assert!(a_square_b > b_square && b_square < a_square_b);
    assert!(a_square_b < a_square_b_plus_a && a_square_b_plus_a > a_square_b);
    assert!(a_square_b < a_square_b_plus_b && a_square_b_plus_b > a_square_b);
    assert!(a_square_b_plus_b < a_square_b_plus_a && a_square_b_plus_a > a_square_b_plus_b);

    assert!(a_plus_b > a && a < a_plus_b);
    assert!(a_plus_b > b && b < a_plus_b);
    assert!(a_plus_b < a_square && a_square > a_plus_b);
    assert!(a_plus_b > b_square && b_square < a_plus_b);

    assert!(a_plus_b_square > a_square && a_square < a_plus_b_square);
    assert!(a_plus_b_square < a_square_b_plus_a && a_square_b_plus_a > a_plus_b_square);
    assert!(a_plus_b_square < a_square_b_plus_b && a_square_b_plus_b > a_plus_b_square);
}

#[test]
pub fn mul_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &(&(&a * &b) + &(&a * &a)) + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &(&(&a * &b) + &(&b * &b)) + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;

    assert!(product.monomials.len() == 7);
    assert!(product.monomials[0].coefficient == 1);
    assert!(product.monomials[0].powers ==
            vec![(Composite::Variable("a".into()), 3), (Composite::Variable("b".into()), 1)]);
    assert!(product.monomials[1].coefficient == 2);
    assert!(product.monomials[1].powers ==
            vec![(Composite::Variable("a".into()), 2), (Composite::Variable("b".into()), 2)]);
    assert!(product.monomials[2].coefficient == 2);
    assert!(product.monomials[2].powers == vec![(Composite::Variable("a".into()), 2)]);
    assert!(product.monomials[3].coefficient == 1);
    assert!(product.monomials[3].powers ==
            vec![(Composite::Variable("a".into()), 1), (Composite::Variable("b".into()), 3)]);
    assert!(product.monomials[4].coefficient == 3);
    assert!(product.monomials[4].powers ==
            vec![(Composite::Variable("a".into()), 1), (Composite::Variable("b".into()), 1)]);
    assert!(product.monomials[5].coefficient == 1);
    assert!(product.monomials[5].powers == vec![(Composite::Variable("b".into()), 2)]);
    assert!(product.monomials[6].coefficient == 2);
    assert!(product.monomials[6].powers.len() == 0);
}

#[test]
pub fn div_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &(&(&a * &b) + &(&a * &a)) + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &(&(&a * &b) + &(&b * &b)) + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;
    // (ab + a^2 + 1) = a * (a + b) + 1
    let (a_plus_b, one) = ab_plus_a_square_plus_one.div_rem(&a);

    assert!(&product / &ab_plus_a_square_plus_one == ab_plus_b_square_plus_two);
    assert!(&product / &ab_plus_b_square_plus_two == ab_plus_a_square_plus_one);

    assert!(a_plus_b == &a + &b);
    assert!(one == 1);
    assert!(product.checked_div(&(&a * &a)).is_none());
    assert!(product.checked_div(&(&b * &b)).is_none());
    assert!(product.checked_div(&2.into()).is_none());
    assert!(product.checked_div(&1.into()).unwrap() == product);
}

#[test]
pub fn add_test() {
    let a_mon = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    // a + b + 1
    let a_plus_b_plus_1_v1 = &(&a + &b) + 1;
    let a_plus_b_plus_1_v2 = &(&a_mon + &b) + 1;
    // 2a + 2b + 2
    let a_plus_b_plus_1_times_2 = &a_plus_b_plus_1_v1 + &a_plus_b_plus_1_v2;

    assert!(a_plus_b_plus_1_v1.monomials.len() == 3);
    assert!(a_plus_b_plus_1_v1.monomials[0].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[0].powers == vec![(Composite::Variable("a".into()), 1)]);
    assert!(a_plus_b_plus_1_v1.monomials[1].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[1].powers == vec![(Composite::Variable("b".into()), 1)]);
    assert!(a_plus_b_plus_1_v1.monomials[2].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[2].powers.len() == 0);
    assert!(a_plus_b_plus_1_v1 == a_plus_b_plus_1_v2);

    assert!(a_plus_b_plus_1_times_2.monomials.len() == 3);
    assert!(a_plus_b_plus_1_times_2.monomials[0].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[0].powers == vec![(Composite::Variable("a".into()), 1)]);
    assert!(a_plus_b_plus_1_times_2.monomials[1].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[1].powers == vec![(Composite::Variable("b".into()), 1)]);
    assert!(a_plus_b_plus_1_times_2.monomials[2].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[2].powers.len() == 0);
}

#[test]
pub fn sub_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    // a + b + 1
    let a_plus_b_plus_1 = &(&a + &b) + 1;
    // 2a + 2b + 2
    let a_plus_b_plus_1_times_2 = 2 * &a_plus_b_plus_1;

    assert!(&a_plus_b_plus_1_times_2 - &a_plus_b_plus_1 == a_plus_b_plus_1);
    assert!(&a_plus_b_plus_1 - 1 == &a + &b);
    assert!(&a_plus_b_plus_1 - &a == &b + 1);
    assert!(&a_plus_b_plus_1 - &b == &a + 1);
    assert!(&a_plus_b_plus_1 - &(&a + &b) == 1);
}

#[test]
pub fn eval_test() {
    let a: TestPolynomial = primitive("a".into());
    let b: TestPolynomial = primitive("b".into());
    let c: TestPolynomial = primitive("c".into());

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 13);

    // a + b + 1
    let a_plus_b_plus_1 = &(&a + &b) + 1;
    assert!(a_plus_b_plus_1.evaluate(&values) == Ok(17));

    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &(&(&a * &b) + &(&a * &a)) + 1;
    assert!(ab_plus_a_square_plus_one.evaluate(&values) == Ok(49));

    // a + b + c + 1
    let a_plus_b_plus_c_plus_1 = &(&a + &b) + &(&c + 1);
    assert!(a_plus_b_plus_c_plus_1.evaluate(&values) == Err("c".into()));

    // ab + bc + c^2
    let ab_plus_bc_plus_c_square = &(&b * &(&a + &c)) + &(&c + &c);
    assert!(ab_plus_bc_plus_c_square.evaluate(&values) == Err("c".into()));
}
