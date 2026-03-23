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
    // usize and range
    // for loop vector
    // iter
    // return value
    // labels
}
