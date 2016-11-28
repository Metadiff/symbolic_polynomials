extern crate symints;

use symints::*;


#[test]
pub fn up_to_coefficient_test() {
    let a = SymMonomial::variable(0);
    let b = SymMonomial::variable(1);
    let c = SymMonomial::from(5);
    // a != b
    assert!(!UpToCoefficient::up_to_coefficient(&a, &b));
    // a != 3
    assert!(!UpToCoefficient::up_to_coefficient(&a, 3));
    // a != c
    assert!(!UpToCoefficient::up_to_coefficient(&a, &c));
    // c == 3
    assert!(UpToCoefficient::up_to_coefficient(&c, 3));
    let a2 = 2 * &a;
    let a5 = -5 * &a;
    // a == 2a
    assert!(UpToCoefficient::up_to_coefficient(&a, &a2));
    // a == -5a
    assert!(UpToCoefficient::up_to_coefficient(&a, &a5));
    // 2a == -5a
    assert!(UpToCoefficient::up_to_coefficient(&a2, &a5));
    // 2a != c
    assert!(!UpToCoefficient::up_to_coefficient(&a2, &c));
    // 2a != 3
    assert!(!UpToCoefficient::up_to_coefficient(&a2, 3));
    let a_sq = &a * &a;
    let a_sq_3 = 3 * &a_sq;
    // a^2 != a
    assert!(!UpToCoefficient::up_to_coefficient(&a_sq, &a));
    // a^2 != 2a
    assert!(!UpToCoefficient::up_to_coefficient(&a_sq, &a2));
    // a^2 != -5a
    assert!(!UpToCoefficient::up_to_coefficient(&a_sq, &a5));
    // a^2 == 3a^2
    assert!(UpToCoefficient::up_to_coefficient(&a_sq, &a_sq_3));
}



#[test]
pub fn partial_eq_test() {
    // a^2 == a^2
    let a_v1 = SymMonomial::variable(1);
    let a_sq_v1 = &a_v1 * &a_v1;
    let a_v2 = SymMonomial::variable(1);
    let a_sq_v2 = &a_v2 * &a_v2;
    assert!(a_sq_v1 == a_sq_v2);
    // 2*a^2 != a^2
    let a = SymMonomial::variable(1);
    let a_sq = &a * &a;
    let a_sq2 = 2 * &(&a * &a);
    assert!(a_sq != a_sq2);
    // a^2 != b^2
    let a = SymMonomial::variable(1);
    let a_sq = &a * &a;
    let b = SymMonomial::variable(2);
    let b_sq = &b * &b;
    assert!(a_sq != b_sq);
}
