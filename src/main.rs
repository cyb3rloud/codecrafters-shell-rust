#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();


     //task 2:print error messages for invalid commands

     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
     //now let's print command not found message
     println!("{}: command not found", command.trim());


     //task 3: implement a REPL loop
      loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();
        //trim the command to remove whitespace and newline characters

        let command = command.trim();
        if command.is_empty() {
            continue; // skip empty commands
        }
        println!("{}: command not found", command);
    }


    }
