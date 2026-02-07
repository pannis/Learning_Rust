fn main() {
    let mut counter = 0;


    // use this idea when checking if a thread
    // has completed
    let result = 'counting_up: loop {
        // named the loop counting_up
            counter += 1;
            if counter == 10 {
                break 'counting_up counter * 2;
            }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
