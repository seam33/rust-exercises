/*
    Description: Recursion
*/

fn main() {
    let number = 15;
    println!("Perrin -> P({}) is {}", number, perrin(number));
    println!("Fibonnaci -> F({}) is {}", number, fibonacci(number));
}

fn perrin(n: i32) -> i32{
    if n == 0 {
        return 3;
    } else if n == 1 {
        return 0;
    } else if n == 2 {
        return 2;
    } else {
        return perrin(n-2) + perrin(n-3);
    }
}

fn fibonacci(n: i32) -> i32{
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}
