use std::io;

fn main() {
    // normal arithmetics
    let x: u8 = 9;
    let y: u8 = 10;
    
    let z = x + y;
    // if we add more than what the data type can handle
    // there would be an overflow for instance u8 has upto 255
    // if we add more than that it will get an overflow error
    // so we have to type cast

    println!("{}", z);

    // type casting and conversion
    // Explicit type conversion
    let s = 127_000 as i32;
    let m = 10;

    let p = s / (m as i32);
    println!("{}", p);

    // how about we try and fetch the inputs from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expect  to read line");
    
    //converting the string to integer
    let int_input: i64 = input.trim().parse().unwrap();
    
    println!("{}", int_input + 2);


}
