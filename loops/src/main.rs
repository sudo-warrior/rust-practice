fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here and the
      // mutable references is allowed to go ahead.
    let r2 = &mut s;

    println!("{}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



//fn main1() {
    // for references and borrowing
    // println!("Hello world!");      
 //   let s1 = String::from("hello");
    
 //   let len = calculate_length(&s1);

   // println!("The length of '{s1}' is {len}.");
//}

//fn calculate_length(s: &String) -> usize {
  //  s.len()
//}
