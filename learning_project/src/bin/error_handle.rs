#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    Other,
}

fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(x / y)
}

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

    let x = 1;
    let y = 19;
    // result<T, E> = Ok(T) | Err(E)
    match div(x, y) {
        Ok(val) => println!("Value is {}", val),
        Err(e) => println!("Error is {:?}", e),
    }
}
