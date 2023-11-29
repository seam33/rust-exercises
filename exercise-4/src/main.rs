/*
    Description: Loops
*/

fn main() {    
    // ---------- Infinite loop ------- //
    println!("\nInfinite Loop");
    let mut count = 0;

    loop {
        if count == 5 {
            println!("Count is {}", count);
            break;
        }        
        count+=1;
    }

    println!("\nFor Loop");
    // ---------- For loop ------- //
    for i in 0..5 {
        println!("Number is {}", i);
    }

    // ---------- While loop ------- //
    println!("\nWhile Loop");
    let mut count2 = 0;

    while count2 < 3 {
        println!("{count2}");
        count2 += 1;
    }

    println!("\nWhile let Loop");
    // ---------- While let loop ------- //
    let mut x = vec![1, 2, 3];

    while let Some(y) = x.pop() {
        println!("y = {}", y);
    }
}
