#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
  
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
        else if command == "exit" {
            break;
        }
        else {     
       
        println!("{}: command not found", command);
    }

      }
    }
    