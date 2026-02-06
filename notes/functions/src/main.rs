fn main() {
    println!("Hello, world!");

    another_function(5);
}

// looks like rust does not care if my function
// is below where it is called
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
