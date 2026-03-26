#[allow(unused)]

fn main() {
    let x: Option<u32> = Some(123);
    match x {
        Some(v) => println!("value is {v}"),
        _ => {}
    }
    // if let
    if let Some(v) = x {
        println!("if let {v}");
    }

    // let else
}
