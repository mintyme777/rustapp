use std::io;

fn main() {
    println!("Hello world");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    //println!("{}", input);

    let trimmed = input.trim();


    if trimmed.is_empty() {
        println!("Input is empty!");
    } else {
        println!("You entered: {}", trimmed);
    }
    

}







