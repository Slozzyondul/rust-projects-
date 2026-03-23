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
    let n: usize = arr.len();
    for i in 0..n {
        println!("array {}", arr[i]);
    }
    // for loop vector
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // for i in vec {
    //     println!("vector {}", i);
    // }
    // iter
    for x in vec.iter() {
        println!("iter {}", x);
    }

    for x in vec.iter() {
        println!("iter {}", x);
    }
    // return value
    let mut i = 0;
    let z = loop {
        if i == 0 {
            break 99;
        }
        i += 1;
    };
    println!("return loop {z}");

    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }
}
