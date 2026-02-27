#![allow(unused)]

//point struct\
#[derive(Debug)]
struct Point {
    x: f32,
    y: u32,
}

// 3d point struct
struct Point3d(f32, f32, f32);

//netsted struct
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // create
    let p = Point { x: 10.0, y: 20 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    let p3d = Point3d(10.0, 20.0, 30.0);
    println!("point3d = {}, {}, {}", p3d.0, p3d.1, p3d.2);

    let c = Circle {
        center: Point { x: 10.0, y: 20 },
        radius: 30,
    };
    println!("circle = {:?}", c);

    // debug
    // read
    // shortcut
    // copy fields
    // update
}
