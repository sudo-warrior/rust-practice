let User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64
}


fn main() {
    // println!("Hello, world!");
    // structs

    let mut user1 = User{
        active: true,
        username: String::from("someoneusername123"),
        email: String::from("someoneusername123@gmail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
}

//fn hello(s: &str) -> &str{
  //  println!("Hello, world!");
//}
