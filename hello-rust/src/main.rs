mod read_file;
use std::io;

fn main(){
    let greeting = "Welcome to the software!\nPlease choose from the following menu: \n 1. Read from a file. \n 2. Write to a file.".to_string();
    loop {
        println!("{}", greeting);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        

        println!();
        println!();
        match input.trim() {

            "1" => { match read_file::read_file() {
                Ok(()) => (),
                Err(_) => println!("Error reading from file")
            } },
            "3" => break,
            _ => println!("Incorrect Input. Please try again."),
        }
    }
    println!("End of Program.");
}

