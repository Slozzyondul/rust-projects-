#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    // enum

    // attributes - debug
    let red = Color::Red;
    let rgba = Color::Rgba(255, 0, 0, 1.0);
    let hex = Color::Hex("#FF0000".to_string());
    let hsl = Color::Hsl {
        h: 0,
        s: 100,
        l: 50,
    };

    println!("{:?}", red);
    println!("{:?}", rgba);
    println!("{:?}", hex);
    println!("{:?}", hsl);

    // partialeq
    println!("{:?}", red == Color::Red);
    println!("{:?}", rgba == Color::Rgba(255, 0, 0, 1.0));
    println!("{:?}", hex == Color::Hex("#FF0000".to_string()));
    println!(
        "{:?}",
        hsl == Color::Hsl {
            h: 0,
            s: 100,
            l: 50
        }
    );

    // option
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    println!("x: {:?}, y: {:?}", x, y);
    // result
    let result: Result<i32, String> = Ok(5);
    let error: Result<i32, String> = Err("Something went wrong".to_string());
    println!("result: {:?}, error: {:?}", result, error);
}
