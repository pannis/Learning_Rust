fn main() {
    println!("Hello, world!");
    let val = y();
    another_function(val, 'k');
}

// looks like rust does not care if my function
// is below where it is called
fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn y() -> i32{
        let num = 4;
        num + 1
            // the missing ; makes it an expression
            // instead of a statement so it returns 5
}
