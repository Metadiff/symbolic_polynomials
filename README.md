# Symbolic Polynomials
[![Build Status](https://travis-ci.org/Metadiff/symbolic_polynomials.svg?branch=master)](https://travis-ci.org/Metadiff/symbolic_polynomials)
[![Documentation](https://img.shields.io/badge/doc-master-brightgreen.svg)](https://Metadiff.github.io/symbolic_polynomials)
[![License](https://img.shields.io/badge/Licence-Apache2.0-blue.svg)](LICENSE-APACHE)
[![Cargo](http://meritbadge.herokuapp.com/symbolic_polynomials)](https://crates.io/crates/symbolic_polynomials)

A Rust library for manipulation and evaluation of symbolic integer polynomials.

## Template types arguments
All of the symbolic expressions have three template types associated with them. 

   1. *I* - the type that uniquely identifies a single symbolic variable
   2. *C* - the type of the free coefficient in every monomial
   3. *P* - the type of the power used in every monomial

## Overview

The main class you most likely will be using is `Polynomial<I, C, P>`, which 
represents any symbolic polynomial. The easiest way to create single variables
(e.g. like `a`, `b`, `c` ...) is by calling `variable(I id)`. The will be a 
unique identification of the variable. From there you can use standard 
arithmetic operators with both other symbolic expressions and with constants. 

If you want to evaluate a symbolic expression, you can call its `eval` method,
which requires you to specify a mapping from unique identifiers to their assignments.

You can also use automatic deduction to solve a system of equations. 

The ordering of both the polynomials and monomials are based on 
[Graded reverse lexicographic order](https://en.wikipedia.org/wiki/Monomial_order#Graded_reverse_lexicographic_order)
derived from the ordering on `I`. Note that this requires the comparison operators
to be implemented for type `I`.

The library provide implements `Display` to convert any expression to a humanly 
readable format. Additionally the `to_code` method renders powers as repeated 
multiplications, and the output string would look like code snippet. 

One drawback is that all of the operators are defined for references.

## Installation

Just add the dependency in your `Cargo.toml` file and then import the crate.
 
## Example usage

Below is the code for a the example found in the `examples` folder.

```rust
use std::collections::HashMap;

extern crate symbolic_polynomials;
use symbolic_polynomials::*;

type SymInt = Polynomial<String, i64, u8>;

pub fn main() {
    // Create symbolic variables
    let a: SymInt = variable("a".into());
    let b: SymInt = variable("b".into());
    let c: SymInt = variable("c".into());

    // Build polynomials
    // 5b + 2
    let poly1 = 5 * &b + 2;
    // ab
    let poly2 = &a * &b;
    // ab + ac + b + c
    let poly3 = &a * &b + &a * &c + &b + &c;
    // a^2 - ab + 12
    let poly4 = &a * &a - &a * &b + 12;
    // ac^2 + 3a + bc^2 + 3b + c^2 + 3
    let poly5 = &a * &c * &c + 3 * &a + &b * &c * &c + 3 * &b + &c * &c + 3;
    // floor(a^2, b^2)
    let poly6 = floor(&(&a * &a), &(&b * &b));
    // ceil(a^2, b^2)
    let poly7 = ceil(&(&a * &a), &(&b * &b));
    // min(ab + 12, ab + a)
    let poly8 = min(&(&a * &b + 12), &(&a * &b + &a));
    // max (ab + 12, ab + a)
    let poly9 = max(&(&a * &b + 12), &(&a * &b + &a));
    // max(floor(a^2, b) - 4, ceil(c, b) + 1)
    let poly10 = max(&(floor(&(&a * &a), &b) - 2), &(ceil(&c, &b) + 1));

    // Polynomial printing
    println!("{}", (0..50).map(|_| "=").collect::<String>());
    println!("Displaying polynomials (string representation = code representation):");
    println!("{} = {}", poly1, poly1.to_code(&|x: String| x));
    println!("{} = {}", poly2, poly2.to_code(&|x: String| x));
    println!("{} = {}", poly3, poly3.to_code(&|x: String| x));
    println!("{} = {}", poly4, poly4.to_code(&|x: String| x));
    println!("{} = {}", poly5, poly5.to_code(&|x: String| x));
    println!("{} = {}", poly6, poly6.to_code(&|x: String| x));
    println!("{} = {}", poly7, poly7.to_code(&|x: String| x));
    println!("{} = {}", poly8, poly8.to_code(&|x: String| x));
    println!("{} = {}", poly9, poly9.to_code(&|x: String| x));
    println!("{} = {}", poly10, poly10.to_code(&|x: String| x));
    println!("{}", (0..50).map(|_| "=").collect::<String>());

    // Polynomial evaluation
    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 2);
    values.insert("c".into(), 5);
    println!("Evaluating for a = 3, b = 2, c = 5.");
    println!("{} = {} [Expected 12]", poly1, poly1.eval(&values).unwrap());
    println!("{} = {} [Expected 6]", poly2, poly2.eval(&values).unwrap());
    println!("{} = {} [Expected 28]", poly3, poly3.eval(&values).unwrap());
    println!("{} = {} [Expected 15]", poly4, poly4.eval(&values).unwrap());
    println!("{} = {} [Expected 78]", poly5, poly5.eval(&values).unwrap());
    println!("{} = {} [Expected 0]", poly6, poly6.eval(&values).unwrap());
    println!("{} = {} [Expected 1]", poly7, poly7.eval(&values).unwrap());
    println!("{} = {} [Expected 9]", poly8, poly8.eval(&values).unwrap());
    println!("{} = {} [Expected 18]", poly9, poly9.eval(&values).unwrap());
    println!("{} = {} [Expected 4]", poly10, poly10.eval(&values).unwrap());
    println!("{}", (0..50).map(|_| "=").collect::<String>());

    // Variable deduction
    values.insert("a".into(), 5);
    values.insert("b".into(), 3);
    values.insert("c".into(), 8);
    let implicit_values = vec![(poly1.clone(), poly1.eval(&values).unwrap()),
                               (poly2.clone(), poly2.eval(&values).unwrap()),
                               (poly3.clone(), poly3.eval(&values).unwrap())];
    let deduced_values = deduce_values(&implicit_values).unwrap();
    println!("Deduced values:");
    println!("a = {} [Expected 5]", deduced_values["a"]);
    println!("b = {} [Expected 3]", deduced_values["b"]);
    println!("c = {} [Expected 8]", deduced_values["c"]);
    println!("{}", (0..50).map(|_| "=").collect::<String>());
}
```

The output of the program:
```rust
==================================================
Displaying polynomials (string representation = code representation):
5b + 2 = 5 * b + 2
ab = a * b
ab + ac + b + c = a * b + a * c + b + c
a^2 - ab + 12 = a * a - a * b + 12
ac^2 + 3a + bc^2 + 3b + c^2 + 3 = a * c * c + 3 * a + b * c * c + 3 * b + c * c + 3
floor(a^2, b^2) = floor(a * a, b * b)
ceil(a^2, b^2) = ceil(a * a, b * b)
min(ab + 12, ab + a) = min(a * b + 12, a * b + a)
max(ab + 12, ab + a) = max(a * b + 12, a * b + a)
max(floor(a^2, b) - 2, ceil(c, b) + 1) = max(floor(a * a, b) - 2, ceil(c, b) + 1)
==================================================
Evaluating for a = 3, b = 2, c = 5.
5b + 2 = 12 [Expected 12]
ab = 6 [Expected 6]
ab + ac + b + c = 28 [Expected 28]
a^2 - ab + 12 = 15 [Expected 15]
ac^2 + 3a + bc^2 + 3b + c^2 + 3 = 168 [Expected 78]
floor(a^2, b^2) = 2 [Expected 0]
ceil(a^2, b^2) = 3 [Expected 1]
min(ab + 12, ab + a) = 9 [Expected 9]
max(ab + 12, ab + a) = 18 [Expected 18]
max(floor(a^2, b) - 2, ceil(c, b) + 1) = 4 [Expected 4]
==================================================
Deduced values:
a = 5 [Expected 5]
b = 3 [Expected 3]
c = 8 [Expected 8]
==================================================
```

You can check out the tests in the `tests` folder for more examples.

## Limitations

Currently, the automatic deduction for solving system of equations 
is pretty limited. The main reason is that for the purposes that the 
project has been developed it is sufficient. A more powerful and complete
algorithm would probably use 
[Grobner basis](https://en.wikipedia.org/wiki/Gr%C3%B6bner_basis).

## License
Symbolic Polynomials is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details.

