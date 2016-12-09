extern crate symints;

type SymInt = symints::Polynomial;
fn main(){
    let mut registry = symints::Registry{id: 0};
    let a = registry.new_monomial_variable();
    let b = registry.new_monomial_variable();
    let ab2 =  &(2 * &a) * &b;
    let asqb = &b * &(&a * &a);
    let composite = &ab2 * &asqb;
    println!("{}", composite);
    let mut div = &composite / &ab2;
    println!("{}", div);
    div = &div / &asqb;
    println!("{}", div);
    let c = &asqb + &composite;
    println!("c={}", c);
    println!("c+c={}", &c + &c);
    let csq = &c * &c;
    println!("c^2={}", csq);
    let mut div2 = &c / &asqb;
    println!("{}", div2);
    div2 = &csq / &(&c * &c);
    println!("{}", div2);
}
