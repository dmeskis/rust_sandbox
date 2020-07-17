fn main() {
    println!("Hello, world!");
    another_function(5, 6)
}


fn another_function(x: i32, y: i8) {
    println!("The value of the argument x is {}", x);
    println!("The value of the argument y is {}", y);
}