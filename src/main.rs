extern crate symints;

type SymInt = symints::SymPolynomial;
fn main(){
    let a = symints::SymMonomial::variable(0);
    let b = symints::SymMonomial::variable(1);
    let ab2 =  &(2 * &a) * &b;
    let asqb = &b * &(&a * &a);
    let composite = &ab2 * &asqb;
    println!("{}", composite);
    println!("{:?}", composite);
    let c = &asqb + &composite;
    println!("c={}", c);
    println!("{:?}", c);
    let csq = &c * &c;
    println!("c^2={}", csq);
    println!("{:?}", csq);
    let mut div = (&composite / &ab2).unwrap();
    println!("{}", div);
    div = (&div / &asqb).unwrap();
    println!("{}", div);
}
