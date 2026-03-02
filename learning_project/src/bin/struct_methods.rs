#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: i32,
}

// struct methods

impl Point {
    // associated functions - static methods
    fn zero() -> Self {
        Self { x: 0.0, y: 0 }
    }

    // methods
    fn move_to(&mut self, x: f32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // distance between two points
    fn dist(&self) -> f32 {
        (self.x * self.x + (self.y * self.y) as f32).sqrt()
    }
}

fn main() {
    let mut p = Point::zero();
    println!("Point: {:?}", p);
    println!("Distance from origin: {}", p.dist());

    p.move_to(3.0, 4);
    println!("New Point: {:?}", p);
    println!("New distance from origin (expected 5.0): {}", p.dist());
}
