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
    let x = 1.0;
    let y = 2;
    let p = Point { x, y };
    println!("point = {:?}", p);
    // copy fields
    let p0 = Point { x: 1.0, y: 2 };
    let p1 = Point { x: 111.0, ..p0 };

    println!("point0 = {:?}, point1 = {:?}", p0, p1);
    // update
    let mut p2 = Point { x: 1.0, y: 2 };
    p2.x += 15.0;
    p2.y += 15;
    println!("point2 = {:?}", p2);
}
