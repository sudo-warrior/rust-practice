
fn fibonacci(n: u64) -> u64 {
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
    //control flow of a program
//    let mut count = 0;
  //  'counting_up: loop {
    //    println!("count = {count}");
     //   let mut remaining = 10;

     //   loop {
      //      println!("remaining: {remaining}");
       //     if remaining == 9 {
         //       break;
          //  }
            //if count == 2 {
              //  break 'counting_up;
            //}
            //remaining -= 1;
       // }
        //count += 1;
   // }
   // println!("End count: {count}");
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut previous = 0u64;
    let mut current = 1u64;

    for _ in 2..=n {
        let next = previous + current;
        previous = current;
        current = next;
    }
    current
    
}

fn main() {
    let n = 60;
    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
}
