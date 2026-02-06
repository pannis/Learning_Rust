use std::io;

fn main() {
    // need to help the compiler sometimes since its static types
    let tup = (500, 6.4, 1);

    // you can use '_' as a prefix to get rid of unused warnings
    let (_x, y, _z) = tup;
    let indexed_x = tup.0;
    println!("The value of y is: {y}");
    println!("The value of x is: {indexed_x}");

    let _unit = ();

    // arrays in rust have fixed length and must be the same type
    // allocated on the stack?
    let _a: [i32; 5]= [1, 2, 3, 4, 5];
    // will learn later of vectors but they can grow and shrink
    //
    // to do an array of the same value do this
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    
    // indexed the same as other languages
    let first_a = a[0];
    println!("first index of a is: {first_a}");


    // index out of bounds example
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
