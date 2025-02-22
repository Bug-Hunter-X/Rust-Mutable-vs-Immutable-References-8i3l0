# Rust Mutable vs Immutable References
This example demonstrates a common error in Rust when working with mutable and immutable references. The core issue lies in attempting to modify a value via an immutable reference, which Rust's ownership system prohibits to maintain memory safety.  The solution highlights how to correctly manage references to avoid this error.

## How to Run
1. Save the code in `bug.rs` and `bugSolution.rs`.
2. Compile and run using `rustc bug.rs && ./bug` and `rustc bugSolution.rs && ./bugSolution`.

## License
MIT