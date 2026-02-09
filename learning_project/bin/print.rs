#![allow(unused)]

fn main() {
    let lang = "Rust";
    println!("The value of lang is: {}", lang);
    println!("hello {} {}", lang, lang);

    let g = 3;
    println!("{0} x {0} = {1}", g, g * g);
}
