fn main() {
    //type annotation
    let mut count: i64 = 0;
    
    // the loop 
    'counting_loop: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("The end count: {count}");

    // the while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}
