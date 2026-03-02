#![allow(unused)]

enum Animal {
    Dog,
    Cat,
    Fish,
    Bird,
}
//
fn main() {
    let x = 187;
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
    let x = 386;
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
    let x = -5;
    match x {
        1..=5 => println!("one or two or three or four or five"),
        _ => println!("something else"),
    }

    // @
    match x {
        i @ 1..=5 => println!("{}", i),
        _ => println!("something else"),
    }

    // return value
    let animal = Animal::Cat;
    let animal_sound = match animal {
        Animal::Dog => "woof",
        Animal::Cat => "meow",
        Animal::Fish => "blub",
        Animal::Bird => "chirp",
        _ => "research bana",
    };
    println!("{}", animal_sound);

    // option
    let x: Option<i32> = Some(5);
    match x {
        Some(val) => println!("{}", val),
        None => println!("nothing"),
    }

    // result
    let x: Result<i32, &str> = Ok(5);
    match x {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}
