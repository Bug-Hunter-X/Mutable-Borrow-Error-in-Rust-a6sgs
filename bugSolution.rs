fn main() {
    let mut x = 5;
    { // this scope helps to solve the error, after this scope ends z becomes unavailable 
        let y = &mut x; // y is a mutable reference to x
        *y = 10;       // Modifies x through y
        println!("x = {}", x); // Prints 10
    }
    let z = &x;   // z is an immutable reference to x
    println!("z = {}", *z); // Prints 10
    let y = &mut x; // y is a mutable reference to x
    *y = 12; //this line no longer causes an error because the scope has ended   
    println!("x = {}", x); // Prints 12
} 