fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x
    *y += 1; // Increment x through y
    *z += 1; // This will lead to a data race and undefined behavior
}