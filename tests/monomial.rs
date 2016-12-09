extern crate symints;

use symints::*;

#[test]
pub fn constructor() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let minus_six = symints::Monomial::from(-6);
    let thirteen = symints::Monomial::from(13);

    assert!(minus_six.is_constant());
    assert!(minus_six.coefficient == -6);
    assert!(minus_six.powers.len() == 0);

    assert!(thirteen.is_constant());
    assert!(thirteen.coefficient == 13);
    assert!(thirteen.powers.len() == 0);

    assert!(!a.is_constant());
    assert!(a.coefficient == 1);
    assert!(a.powers == vec![(0, 1)]);
}

#[test]
pub fn up_to_coefficient_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    let three = Monomial::from(3);
    let five = Monomial::from(5);
    // 2a
    let a_times_2 = 2 * &a;
    // -5a
    let minus_5_a = -5 * &a;
    // a^2
    let a_square = &a * &a;
    // 3a^2
    let a_square_times_3 = 3 * &a_square;

    assert!(!a.up_to_coefficient(&three) && !three.up_to_coefficient(&a));
    assert!(!a.up_to_coefficient(&five) && !five.up_to_coefficient(&a));
    assert!(!b.up_to_coefficient(&three) && !three.up_to_coefficient(&b));
    assert!(!b.up_to_coefficient(&five) && !five.up_to_coefficient(&b));

    assert!(!a.up_to_coefficient(&b) && !b.up_to_coefficient(&a));

    assert!(five.up_to_coefficient(&three) && three.up_to_coefficient(&five));
    assert!(a.up_to_coefficient(&a_times_2) && a_times_2.up_to_coefficient(&a));
    assert!(a.up_to_coefficient(&minus_5_a) && minus_5_a.up_to_coefficient(&a));
    assert!(a_times_2.up_to_coefficient(&minus_5_a) && minus_5_a.up_to_coefficient(&a_times_2));

    assert!(!a_times_2.up_to_coefficient(&three) && !three.up_to_coefficient(&a_times_2));
    assert!(!a_times_2.up_to_coefficient(&five) && !five.up_to_coefficient(&a_times_2));

    assert!(!a_square.up_to_coefficient(&a) && !a.up_to_coefficient(&a_square));
    assert!(!a_square.up_to_coefficient(&a_times_2) && !a_times_2.up_to_coefficient(&a_square));
    assert!(!a_square.up_to_coefficient(&minus_5_a) && !minus_5_a.up_to_coefficient(&a_square));
    assert!(a_square.up_to_coefficient(&a_square_times_3) && a_square_times_3.up_to_coefficient(&a_square));
}

#[test]
pub fn partial_eq_test() {
    let mut registry = symints::Registry::default();
    let a_v1 = registry.new_monomial_variable();
    let a_v2 = registry.specific_monomial_variable(0);
    let b = registry.new_monomial_variable();
    // a^2
    let a_square_v1 = Monomial{coefficient: 1, powers: vec![(0, 2)]};
    let a_square_v2 = Monomial{coefficient: 1, powers: vec![(0, 2)]};
    // 2a^2
    let two_a_square = Monomial{coefficient: 2, powers: vec![(0, 2)]};
    // b^2
    let b_square = &b * &b;

    assert!(a_v1 != 3 && 3 != a_v1);
    assert!(a_v1 == a_v2 && a_v2 == a_v1);

    assert!(a_square_v1 == a_square_v2 && a_square_v2 == a_square_v1);
    assert!(two_a_square != a_square_v1 && a_square_v1 != two_a_square);
    assert!(a_square_v1 != b_square && b_square != a_square_v1);
}

#[test]
pub fn ord_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    // 2a^2b
    let a_square_b_times_2 = Monomial{coefficient: 2, powers: vec![(0, 2), (1, 1)]};
    // 3a^2b
    let a_square_b_times_3 = Monomial{coefficient: 3, powers: vec![(0, 2), (1, 1)]};
    // 3ab
    let ab_times_3 = Monomial{coefficient: 2, powers: vec![(0, 1), (1, 1)]};

    assert!(a > 2 && 2 < a);
    assert!(b > 2 && 2 < b);

    assert!(a > b && b < a);
    assert!(a_square_b_times_3 > a_square_b_times_2 && a_square_b_times_2 < a_square_b_times_3);
    assert!(a_square_b_times_3 > ab_times_3 && ab_times_3 < a_square_b_times_3);
    assert!(a_square_b_times_2 > ab_times_3 && ab_times_3 < a_square_b_times_2);
}

#[test]
pub fn mul_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    let c = registry.new_monomial_variable();
    // 2abc
    let abc_times_2 = &(2 * &a) * &(&b * &c);
    // b^2
    let b_square = &b * &b;
    // 2ab^3c
    let ab_third_c_times_2 = &b_square * &abc_times_2;

    assert!(abc_times_2.coefficient == 2);
    assert!(!abc_times_2.is_constant());
    assert!(abc_times_2.powers == vec![(0, 1), (1, 1), (2, 1)]);

    assert!(b_square.coefficient == 1);
    assert!(b_square.powers == vec![(1, 2)]);

    assert!(ab_third_c_times_2.coefficient == 2);
    assert!(ab_third_c_times_2.powers == vec![(0, 1), (1, 3), (2, 1)]);
}

#[test]
pub fn div_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    let c = registry.new_monomial_variable();
    let d = registry.new_monomial_variable();
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // c^2
    let c_square = &c * &c;
    // 2abc
    let abc_times_2 = &(2 * &a) * &(&b * &c);
    // ab
    let ab = &a * &b;
    // ac
    let ac = &a * &c;
    // bc
    let bc = &b * &c;

    // abc
    let abc= &abc_times_2 / 2;
    assert!(abc.coefficient == 1);
    assert!(abc.powers == vec![(0, 1), (1, 1), (2, 1)]);

    // 2bc
    let bc2 = &abc_times_2 / &a;
    assert!(bc2.coefficient == 2);
    assert!(bc2.powers == vec![(1, 1), (2, 1)]);

    // 2ac
    let ac2 = &abc_times_2 / &b;
    assert!(ac2.coefficient == 2);
    assert!(ac2.powers == vec![(0, 1), (2, 1)]);

    // 2ab
    let ab2 = &abc_times_2 / &c;
    assert!(ab2.coefficient == 2);
    assert!(ab2.powers == vec![(0, 1), (1, 1)]);

    // 2
    let two = &(&(&abc_times_2 / &b) / &c) / &a;
    assert!(two.coefficient == 2);
    assert!(two.powers == vec![]);

    // 2c
    let c2 = &abc_times_2 / &ab;
    assert!(c2.coefficient == 2);
    assert!(c2.powers == vec![(2, 1)]);

    // 2b
    let b2 = &abc_times_2 / &ac;
    assert!(b2.coefficient == 2);
    assert!(b2.powers == vec![(1, 1)]);

    // 2a
    let a2 = &abc_times_2 / &bc;
    assert!(a2.coefficient == 2);
    assert!(a2.powers == vec![(0, 1)]);

    assert!(abc_times_2.checked_div(4).is_none());
    assert!(abc_times_2.checked_div(&a_square).is_none());
    assert!(abc_times_2.checked_div(&b_square).is_none());
    assert!(abc_times_2.checked_div(&c_square).is_none());
    assert!(abc_times_2.checked_div(&d).is_none());
}

#[test]
pub fn add_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();

    // a + b
    let a_plus_b = &a + &b;
    assert!(a_plus_b.monomials.len() == 2);
    assert!(a_plus_b.monomials[0] == a);
    assert!(a_plus_b.monomials[1] == b);

    // a + 2b
    let a_plus_two_b = &a_plus_b + &b;
    assert!(a_plus_b.monomials.len() == 2);
    assert!(a_plus_two_b.monomials[0] == a);
    assert!(a_plus_two_b.monomials[1] == 2 * &b);

    // 2a + 2b
    let a_plus_b_twice = &a_plus_two_b + &a;
    assert!(a_plus_b.monomials.len() == 2);
    assert!(a_plus_b_twice.monomials[0] == 2 * &a);
    assert!(a_plus_b_twice.monomials[1] == 2 * &b);

    // 0
    let zero = &a_plus_b + &(-&a_plus_b);
    assert!(zero.monomials.len() == 0);
}

#[test]
pub fn sub_test() {
    let mut registry = symints::Registry::default();
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();

    // a - b
    let a_minus_b = &a - &b;
    assert!(a_minus_b.monomials.len() == 2);
    assert!(a_minus_b.monomials[0] == a);
    assert!(a_minus_b.monomials[1] == -&b);

    // a - 2b
    let a_minus_two_b = &a_minus_b - &b;
    assert!(a_minus_two_b.monomials.len() == 2);
    assert!(a_minus_two_b.monomials[0] == a);
    assert!(a_minus_two_b.monomials[1] == -2 * &b);

    // a
    let a_v2 = &a_minus_two_b + &(2 * &b);
    assert!(a_v2.monomials.len() == 1);
    assert!(a_v2.monomials[0] == a);

    // 0
    let zero = &a_v2 + &(-&a);
    assert!(zero.monomials.len() == 0);
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
