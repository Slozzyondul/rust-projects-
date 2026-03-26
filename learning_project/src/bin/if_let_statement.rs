#[allow(unused)]

fn main() {
    //let x: Option<u32> = Some(123);
    let x: Option<u32> = None;
    match x {
        Some(v) => println!("value is {v}"),
        _ => {}
    }
    // if let
    if let Some(v) = x {
        println!("if let {v}");
    }

    // let else
    let Some(v) = x else {
        // diverge - panic or return
        panic!("x is not Some");
    };
    println!("v = {v}");
}
