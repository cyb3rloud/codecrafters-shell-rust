#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();


     //task 2:print error messages for invalid commands

     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
     //now let's print command not dound message
     println!("{}: command not found", command.trim());
}
