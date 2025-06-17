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
    let mut s = String::from("world");
    s = String::from("Ahoy");

    println!("{s}, world!", s);
}
