#![allow(unused)]

fn main() {
    // type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (1 as u32);
    // min and max
    let min = i32::MIN;
    let max = i32::MAX;
    println!("The value of min is: {}", min);
    println!("The value of max is: {}", max);
    // overflow
    let mut u: u32 = u32::MAX;
    //u += 1;
}
