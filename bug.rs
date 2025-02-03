fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10;       // Modifies x through y
    let z = &x;   // z is an immutable reference to x
    println!("x = {}", x); // Prints 10
    println!("z = {}", *z); // Prints 10
    //this next line causes an error 
    *y = 12; //this causes an error because the reference z prevents the mutable reference y from modifing x 
}