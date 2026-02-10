#![allow(unused)]

fn main() {
    // array - fixed length, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[2] = {}", arr[2]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[2] = 4;
    println!("arr[2] = {}", arr[2]);

    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);
    // slice - dynamic length, known at runtime
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    // first three elements
    let slice = &nums[0..3];
    println!("{:?}", slice);
    // last three elements
    let slice = &nums[7..10];
    println!("{:?}", slice);
    // middle three elements
    let slice = &nums[3..6];
    println!("{:?}", slice);
    // all elements
    let slice = &nums[0..10];
    println!("{:?}", slice);
}
