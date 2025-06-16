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

    // looping through a collection with for loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // a more concise way to loop for the above loop
    for element in a {
        println!("The value is: {element}");
    }


    // using range in rust for the loping functions
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
