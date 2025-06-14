// for accepting inputs
use std::io;

fn main() {
    // constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);

    // gloal sccope
    let x = 4;
    println!("The value of x is: {}", x);

    {   
        // Local scope
        let x = x - 2;
        println!("The value of x is: {}", x);
    }
    
    // the changes of the global scope variable
    let x = x + 1;
    println!("The value of x is: {}", x);

    // turples
    let mut tup: (i32, bool, char) = (1, true, 's');
    
    println!("{}", tup.0);
    tup = (10, false, 'a');

    println!("{}", tup.0); 

    // explicit assignment
    // let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; 

    // Arrays
    let mut arr = [1, 2, 3, 4, 5];
    arr[4] = 3;

    println!("{}", arr[4]);

    // accepting inputs
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");


    println!("{}", input);
}

