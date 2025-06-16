use std::io;

fn main() {
    // trying out the fibonnacci series
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Expected a number");

    // change the string back to number
    let result:usize = input.trim().parse().expect("expected a number");

    //  trial for the fibonacci series
   let mut fibo:Vec<u32> = Vec::new();
   for number in 0..result{
     if number == 0 {
         fibo.push(0)
     } else if number == 1 {
        fibo.push(1);
     } else {
         let next = fibo[number - 1] + fibo[number - 2];
         fibo.push(next);
     }
    
   }
   println!("The result for the fibonacci series of {} terms is {:?}", result, fibo)
}
