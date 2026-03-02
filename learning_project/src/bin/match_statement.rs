#![allow(unused)]

fn main() {
    let x = 1;
    if x == 1 {
        println!("one");
    } else if x == 2 {
        println!("two");
    } else if x == 3 {
        println!("three");
    } else {
        println!("something else");
    }
    // match
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }
    // multiple cases
    let x = 5;
    match x {
        1 | 2 | 3 => println!("one or two or three"),

        _ => println!("something else"),
    }

    // range

    // @

    // return value

    // option

    // result
}
