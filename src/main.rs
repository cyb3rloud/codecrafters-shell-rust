#[allow(unused_imports)]
use std::io::{self, Write};






fn handle_echo(args: &[&str]) {
             //now, we join the arguments back together with spaces and print them
             println!("{}", args.join(" "));   
            }

fn main() {

   


  
      loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        //trim the command to remove whitespace and newline characters

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() { continue; }


    let command = parts[0];
    let args = &parts[1..];


    match command {
        "exit" | "bye" => break,
        "echo" => handle_echo(args),
        _ => println!("{}: command not found", command)
    }

}
       
        
    }


    