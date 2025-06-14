use std::io;

fn main() {
    // control flow and conditions
    let x = 2 <= 3;
    let cond = false || !x;
    println!("{}", cond);
    

    // if statement
    let food = "mango";

    if food == "cookie" {
        println!("I like Cookies too");
    } else if food == "fruit" {
        test();
    } else {
        test2();
    }


}


fn test() {
    println!("The test function has been called...");
}

fn test2() {
    println!("This is the way it should be now, ooh my God, neovim with nvchad is the best");
}

fn add_two_functions() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("There is an input error");
    println!("{}", input);
   

}
