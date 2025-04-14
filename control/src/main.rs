
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
    //let mut attempts = 0;

    //loop {
     //   println!("connecting");
       // attempts += 1;

       // i/f attempts >= 3 {
         //   println!("Failed after 3 attempts");
           // break;
       // }
    //}
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

}
