#![allow(unused)]

//point struct
struct Point {
    x: f32,
    y: u32,
}

// 3d point struct
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

//netsted struct
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // create
    let p = Point { x: 10.0, y: 20 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    // debug
    // read
    // shortcut
    // copy fields
    // update
}
