# Mutable Borrow Error in Rust

This example demonstrates a common error in Rust: attempting to modify a value through a mutable reference while an immutable reference to the same value also exists.  The Rust compiler prevents this to ensure data integrity and avoid data races. The solution shows how to restructure the code to avoid the conflict.