#![allow(unused)]

fn main() {
    // string = vector of u8 (Vec<u8>)
    // String = growable, heap-allocated, UTF-8 encoded string
    // &str = string slice, pointer to a sequence of bytes in memory

    // when to use which
    // use &str when you don't need to modify the string / read only
    // use String when you need to modify the string / mutable

    // string
    let msg: String = String::from("hello solo");
    let len: usize = msg.len();

    println!("msg: {msg}");
    println!("len: {len}");

    // string slice
    let msg: String = String::from("hello solo");
    let slice: &str = &msg[0..7];
    println!("slice: {slice}");

    // string literal
    // stored inside a binary
    // slice pointing to a specific part of the binary
    // immutable because hard-coded inside binary

    let hello: &str = "hello solo 1";
    println!("{hello}");

    // multiline string with json structure
    let s: &str = r#"{
        "name": "solo",
        "age": 29,
        "city": "nairobi",
        "contact": {
            "email": "solomonondula@gmail.com",
            "phone": "+254792352745"
        }
    }"#;
    println!("{s}");

    // deref coercion
    let msg: String = String::from("hello solo");
    let slice: &str = &msg;
    println!("{slice}");

    // add &str to String
    let mut msg: String = "hello solo".to_string();
    msg += " from kenya";
    println!("{msg}");

    // add String to String
    let mut msg: String = "hello solo".to_string();
    msg += &String::from(" from kenya");
    println!("{msg}");

    // string interpolation
    let name: &str = "solo";
    let age: u8 = 29;
    let msg: String = format!("hello {}, you are {} years old", name, age);
    println!("{msg}");
}
