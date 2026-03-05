#![allow(unused)]

fn main() {
    // +. -, *, /

    let x: i32 = 1;
    let y: i32 = 2;
    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);

    // in rust division of integers rounds down
    println!("x / y = {}", x / y);

    // %  (remainder or mod operation)
    let x: i32 = -1;
    let y: i32 = 2;
    println!("x % y = {}", x % y);

    // literals
    let a = 3i32; // i32 format number (3)
    let b = 1.23e3; // 1.23 x 10^3
    let c = 0x1A; // hex format number (26)
    let d = 0b1010; // binary format number (10)
    let e = 0o755; // octal format number (493)
    let f = 1_000_000_000u32; // 1,000,000,000u32

    // boolean
    let a = true && false; // false
    let b = true || false; // true
    let c = !true; // false

    //bitwise
    //101
    let a: u8 = 5;

    // 011
    let b: u8 = 3;

    // 111
    println!("a & b = {}", a & b);

    // 111
    println!("a | b = {}", a | b);

    // 110
    println!("a ^ b = {}", a ^ b);

    // 10100
    println!("a << 2 = {}", a << 2);

    // 00101
    println!("a >> 2 = {}", a >> 2);
}
