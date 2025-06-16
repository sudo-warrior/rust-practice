use std::io;

fn main() {
    // (32°F − 32) × 5/9 = 0°C
    // prompting the user to input the conversion in 
    // in bidirectional way
    println!("Enter the choice you want for conversion");
    println!("1. for conversion from farenheit to celcius");
    println!("2 for conversion of celcius to ferenheirt");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice = choice.trim();

    // Decide the direction
    match choice {
        "1"=>{
            println!("The conversion from ferenheit to celcius");
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Expected a number");
            let ferenheirt: f64 = input.trim().parse().expect("Number expected");

            // conversion happening
            let celcius: f64 = (ferenheirt - 32.0) * 5.0/9.0;

            println!("The conversion for the {:.2}F is {:.2}C", ferenheirt, celcius)
        }
        "2" => {
            println!("The conversion from celcius to ferenheirt");
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Expected a number");
            let celcius: f64 = input.trim().parse().expect("Number expected");
            
            // conversion happening
            let ferenheirt: f64 = (celcius * 9.0/5.0) + 32.0;

            println!("The conversion of the {:.2}C is {:.2}F", celcius, ferenheirt)
        }
        _ => {
            println!("Invalid types");
        }
    }

}
