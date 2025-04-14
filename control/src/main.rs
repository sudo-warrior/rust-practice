
fn main() {
    // let temperature = 23;

    // let weather = if temperature <= 10 {
       // "cold"
    // } else if temperature <= 20 {
       //  "cool"
    // } else {
       // "warm"
    // };
    
    // println!("The weather  is {}", weather);
    //
    let mut attempts = 0;

    loop {
        println!("connecting");
        attempts += 1;

        if attempts >= 3 {
            println!("Failed after 3 attempts");
            break;
        }
    }
}
