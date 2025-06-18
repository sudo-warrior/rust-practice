fn main() {
    // rust ownership
    let mut s = String::from("hello");
    
    // push_str() appends a literal to a string
    s.push_str(", world!");

    println!("{s}");

    // memory and allocation
    // The memory allocated in data is cleared
    // automatically when it goes out of scope
    // the drop function is automatically called
    // which can sometimes cause unexpected behaviours
    // if we would like to use it beyond normal 
    // usercases

    // Variables and data interacting with moves
    // double free error when the two stacks pointing
    // to the same heap and tr to drop twice the same heap
    // avoided by moving the contents of the heap be pointed
    // to the next stacks rather than mearly copying
    //let mut s = String::from("world");
    let s = String::from("Ahoy");
    let _s1 = take_ownership();
    
    let t1 = String::from("Hello");

    println!("{s}, world!");
    
    let p = String::from("hello");
    let s3 = copy_function(p); // p's value moves into the function
                       // and so is no linger valid Here
    let x = 5;
    // copy_function(x);

    println!("{}", x);

    let (s2, len) = calculate_length(t1);

    println!("The length of '{s2} is {len}")

}

// preview comes into scope
fn take_ownership() {
    let some_string = String::from("hello");
    some_string;
    //  println!("{preview}");
} // preview gets out of scope

// int comes into scope
fn copy_function(a_string: String) {
    // a_string comes into scope
    a_string;
    //println!("{int}");
} // Here, some integer goes out of scope. Nothing special happens

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}
