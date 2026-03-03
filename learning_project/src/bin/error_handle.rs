#![allow(unused)]

fn main() {
    //error
    //panic!("crash and burn");

    // option or result
    let arr = [1, 2, 3];
    //arr[10];

    //option<&i32> = some(&i32) | None
    let x: Option<&i32> = arr.get(2);
    match x {
        Some(val) => println!("Value is {}", val),
        None => println!("Value is None"),
    }
}
