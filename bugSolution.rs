fn main() {
    let mut x = 5;
    let mut y = x; // y is a copy of x
    let mut z = x; // z is another copy of x
    y += 1; // Increment y
    z += 1; // Increment z
    println!("x: {}, y: {}, z: {}", x, y, z);
}
//Alternative solution using Mutex (for more complex scenarios)
use std::sync::{Arc, Mutex};
fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    *y.lock().unwrap() += 1;
    *z.lock().unwrap() += 1;
    println!("x: {}", *x.lock().unwrap());
}