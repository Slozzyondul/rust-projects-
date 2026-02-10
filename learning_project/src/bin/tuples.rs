#![allow(unused)]

fn main() {
    // tuple
    let x: (i32, f64, u8, char) = (500, 6.4, 1, 'a');
    // print tuple
    println!("x = {}, {}, {}, {}", x.0, x.1, x.2, x.3);
    // destructure
    let (a, b, c, d) = x;
    // ignore with _
    let (a, _, _, _) = x;
    // empty tuple - unit type
    let y = ();
    // nested tuple
    let z = (1, (2, 3), 4);
    // print nested tuple
    println!("z = {}, {:?}, {}", z.0, z.1, z.2);
}
