/*
    Description: This is the simplest program in rust. The are some examples
    relatate to strings and numbers.

    Note: By default all variables in rust are inmutable, if you want to change that behavior, 
    it should use "mut".
*/

fn main() {
    // By default all variable in rust are inmutable, if you want to change that behavior, it should use "mut"
    let message = "Hello World";
    let mut number = 25;

    number = 26;

    println!("This's my message -> '{}'", message);
    println!("This's is a number -> '{}'", number);
}