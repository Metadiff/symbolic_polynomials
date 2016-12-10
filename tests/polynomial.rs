extern crate symints;
use symints::*;

#[test]
pub fn constructor() {
    let mut registry = Registry::default();
    let a = registry.new_variable();
    let b = Polynomial::from(&(5 * &registry.specific_monomial_variable(1)));
    let minus_six = Polynomial::from(-6);
    let thirteen = Polynomial::from(13);

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
    assert!(a.monomials[0].powers == vec![(0, 1)]);

    assert!(!b.is_constant());
    assert!(b.monomials.len() == 1);
    assert!(b.monomials[0].coefficient == 5);
    assert!(b.monomials[0].powers == vec![(1, 1)]);
}

#[test]
pub fn partial_eq_test() {
    let mut registry = Registry::default();
    let a = registry.new_variable();
    let b = registry.new_variable();
    let a_v2 = registry.specific_variable(0);
    let b_v2 = registry.specific_variable(1);
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
    let mut registry = Registry::default();
    let a = registry.new_variable();
    let b = registry.new_variable();
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
    let mut registry = Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &(&(&a * &b) + &(&a * &a)) + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &(&(&a * &b) + &(&b * &b)) + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;

    assert!(product.monomials.len() == 7);
    assert!(product.monomials[0].coefficient == 1);
    assert!(product.monomials[0].powers == vec![(0, 3), (1, 1)]);
    assert!(product.monomials[1].coefficient == 2);
    assert!(product.monomials[1].powers == vec![(0, 2), (1, 2)]);
    assert!(product.monomials[2].coefficient == 2);
    assert!(product.monomials[2].powers == vec![(0, 2)]);
    assert!(product.monomials[3].coefficient == 1);
    assert!(product.monomials[3].powers == vec![(0, 1), (1, 3)]);
    assert!(product.monomials[4].coefficient == 3);
    assert!(product.monomials[4].powers == vec![(0, 1), (1, 1)]);
    assert!(product.monomials[5].coefficient == 1);
    assert!(product.monomials[5].powers == vec![(1, 2)]);
    assert!(product.monomials[6].coefficient == 2);
    assert!(product.monomials[6].powers.len() == 0);
}

#[test]
pub fn div_test() {
    let mut registry = Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &(&(&a * &b) + &(&a * &a)) + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &(&(&a * &b) + &(&b * &b)) + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;

    assert!(&product / &ab_plus_a_square_plus_one == ab_plus_b_square_plus_two);
    assert!(&product / &ab_plus_b_square_plus_two == ab_plus_a_square_plus_one);

    assert!(product.checked_div(&(&a * &a)).is_none());
    assert!(product.checked_div(&(&b * &b)).is_none());
    assert!(product.checked_div(2).is_none());
    assert!(product.checked_div(1).unwrap() == product);
}

#[test]
pub fn add_test() {
    let mut registry = Registry::default();
    let a_mon = registry.specific_monomial_variable(0);
    let a = registry.new_variable();
    let b = registry.new_variable();
    // a + b + 1
    let a_plus_b_plus_1_v1 = &(&a + &b) + 1;
    let a_plus_b_plus_1_v2 = &(&a_mon + &b) + 1;
    // 2a + 2b + 2
    let a_plus_b_plus_1_times_2 = &a_plus_b_plus_1_v1 + &a_plus_b_plus_1_v2;

    assert!(a_plus_b_plus_1_v1.monomials.len() == 3);
    assert!(a_plus_b_plus_1_v1.monomials[0].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[0].powers == vec![(0, 1)]);
    assert!(a_plus_b_plus_1_v1.monomials[1].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[1].powers == vec![(1, 1)]);
    assert!(a_plus_b_plus_1_v1.monomials[2].coefficient == 1);
    assert!(a_plus_b_plus_1_v1.monomials[2].powers.len() == 0);
    assert!(a_plus_b_plus_1_v1 == a_plus_b_plus_1_v2);

    assert!(a_plus_b_plus_1_times_2.monomials.len() == 3);
    assert!(a_plus_b_plus_1_times_2.monomials[0].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[0].powers == vec![(0, 1)]);
    assert!(a_plus_b_plus_1_times_2.monomials[1].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[1].powers == vec![(1, 1)]);
    assert!(a_plus_b_plus_1_times_2.monomials[2].coefficient == 2);
    assert!(a_plus_b_plus_1_times_2.monomials[2].powers.len() == 0);
}

#[test]
pub fn sub_test() {
    let mut registry = Registry::default();
    let a = registry.new_variable();
    let b = registry.new_variable();
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
pub fn floor_ceil_test() {
    // TODO
}

#[test]
pub fn min_max_test() {
    // TODO
}

#[test]
pub fn eval_test() {
    // TODO
}

