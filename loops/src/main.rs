fn main() {
    // rust ownership
    let mut s = String::from("hello");
    
    // push_str() appends a literal to a string
    s.push_str(", world!");

    println!("{s}");
}
