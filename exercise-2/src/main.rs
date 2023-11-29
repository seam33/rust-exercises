/*
    Description: Conditionals
*/

fn main() {
    let agreement = true;

    if agreement {
        println!("We will make money together.")
    }else{
        println!("We are not on the same page.")
    }

    // -------------------------- //

    let age = 50;

    if age >= 50 {
        println!("Old")
    }else if age >= 20 {
        println!("Young")
    }else{
        println!("Tennager!")
    }
}
