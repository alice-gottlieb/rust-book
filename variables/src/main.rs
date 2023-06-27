fn main() {
    let mut x = 5; 
    println!("The value of x is: {}", x);
    x = 6; // error: cannot assign twice to immutable variable `x`
    println!("The value of x is: {}", x);
}
