#[allow(unused)]

fn main() {
    // loop
    let mut i = 0;
    loop {
        println!("loop {i}");
        if i == 10 {
            break;
        }
        i += 1;
    }
    // while
    let mut i = 0;
    while i <= 10 {
        println!("while {i}");
        i += 1;
    }
    // for loop
    for i in 0..=10 {
        println!("for {i}");
    }
    // for loop array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for i in arr {
        println!("for {i}");
    }

    // usize and range
    // for loop vector
    // iter
    // return value
    // labels
}
