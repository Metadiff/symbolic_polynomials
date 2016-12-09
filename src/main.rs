extern crate symints;

type SymInt = symints::Polynomial;
fn main(){
    let mut registry = symints::Registry{id: 0};
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    let ab2 =  &(2 * &a) * &b;
    let a_square_b = &b * &(&a * &a);
    let composite = &ab2 * &a_square_b;
    println!("{}", composite);
    let mut div = &composite / &ab2;
    println!("{}", div);
    div = &div / &a_square_b;
    println!("{}", div);
    let c = &a_square_b + &composite;
    println!("c={}", c);
    println!("c+c={}", &c + &c);
    let csq = &c * &c;
    println!("c^2={}", csq);
    let mut div2 = &c / &a_square_b;
    println!("{}", div2);
    div2 = &csq / &(&c * &c);
    println!("{}", div2);
}
