#![allow(unused)]

fn main() {
    println!("Hello, world!");

    //variables are imutable by defaullt in rust
    let x: i32 = 5;
    println!("The value of x is: {}", x);

    //mutable variables
    let mut y: i32 = 6;
    y += 1;
    println!("The value of y is: {}", y);

    //constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    //vectors
    let v: Vec<_> = vec![1, 2, 3, 4, 5];

    //shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
