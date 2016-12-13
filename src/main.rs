extern crate symints;
use std::collections::HashMap;

use symints::*;


type Shape = (Polynomial, Polynomial, Polynomial, Polynomial);

enum ConvolutionMode{
    VALID,
    HALF,
    FULL
}

fn is_2d(shape: &Shape) -> bool {
    shape.2 == 1 && shape.3 == 1
}

fn matrix_mul_shape(left: &Shape, right: &Shape) -> Option<Shape> {
    // Check we have a 2D tensors with matching middle dimension
    if left.1 != right.0 || left.2 != 1 || left.3 != 1 || right.2 != 1 || right.3 != 1 {
        None
    } else {
        Some((left.0.clone(), right.1.clone(), 1.into(), 1.into()))
    }
}

fn element_wise_shape(left: &Shape, right: &Shape) -> Option<Shape> {
    // Check we have a 2D tensors with matching middle dimension
    if left != right {
        None
    } else {
        Some(left.clone())
    }
}

fn convolution_2d_shape(image: &Shape, kernel: &Shape, stride: &Shape,
                        mode: ConvolutionMode) -> Option<Shape> {
    // Check everything is 2D
    if is_2d(image) && is_2d(kernel) && is_2d(stride) {
        let (padding0, padding1) : (Polynomial, Polynomial) = match mode {
            ConvolutionMode::VALID => {
                (0.into(), 0.into())
            },
            ConvolutionMode::HALF => {
                (floor(&kernel.0, &2.into()), floor(&kernel.1, &2.into()))
            }
            ConvolutionMode::FULL => {
                (&kernel.0 - 1, &kernel.1 - 1)
            }
        };
        Some((ceil(&(&(&image.0 - &kernel.0) + &(2 * &padding0)), &stride.0),
             ceil(&(&(&image.1 - &kernel.1) + &(2 * &padding1)), &stride.1),
             1.into(), 1.into()))
    } else {
        None
    }
}

fn eval_shape(shape: &Shape, values: &HashMap<u16, i64>) -> Result<(i64, i64, i64, i64), u16> {
    Ok((try!(shape.0.evaluate(values)),
    try!(shape.1.evaluate(values)),
    try!(shape.2.evaluate(values)),
    try!(shape.3.evaluate(values))))
}

fn main(){
    let a = primitive(0);
    let b = primitive(1);
    let c = primitive(2);
    let d = primitive(3);
    let mut values: HashMap<u16, i64> = HashMap::new();
    values.insert(0, 20);
    values.insert(1, 7);
    values.insert(2, 10);
    values.insert(3, 3);
    let mut temp: Shape;
    let s1: Shape = (a.clone(), b.clone(), 1.into(), 1.into());
    let s2: Shape = (b.clone(), c.clone(), 1.into(), 1.into());
    let s3: Shape = (a.clone(), b.clone(), 1.into(), 1.into());
    let im: Shape = (c.clone(), c.clone(), 1.into(), 1.into());
    let ker: Shape = (d.clone(), d.clone(), 1.into(), 1.into());
    let st: Shape = (2.into(), 2.into(), 1.into(), 1.into());
    temp = matrix_mul_shape(&s1, &s2).unwrap();
    println!("{:?}", temp);
    println!("{:?}", eval_shape(&temp, &values));
    println!("{:?}", matrix_mul_shape(&s1, &s1));
    temp = element_wise_shape(&s1, &s3).unwrap();
    println!("{:?}", temp);
    println!("{:?}", eval_shape(&temp, &values));
    println!("{:?}", element_wise_shape(&s1, &s2));
    temp = convolution_2d_shape(&im, &ker, &st, ConvolutionMode::VALID).unwrap();
    println!("{:?}", temp);
    println!("{:?}", eval_shape(&temp, &values));
    temp = convolution_2d_shape(&im, &ker, &st, ConvolutionMode::HALF).unwrap();
    println!("{:?}", temp);
    println!("{:?}", eval_shape(&temp, &values));
    temp = convolution_2d_shape(&im, &ker, &st, ConvolutionMode::FULL).unwrap();
    println!("{:?}", temp);
    println!("{:?}", eval_shape(&temp, &values));
}
