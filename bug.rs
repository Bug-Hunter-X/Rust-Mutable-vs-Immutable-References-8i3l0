fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y (OK)
    // *z += 1; // ERROR: Cannot assign to immutable reference
    println!("x = {}", x); //Prints 6
}