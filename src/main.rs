#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;



fn handle_echo(args: &[&str]) {
             //now, we join the arguments back together with spaces and print them
             println!("{}", args.join(" "));   
            }
fn handle_type(args: &[&str]){
    if args.is_empty() {
        println!("type is a shell builtin");
        return; //Handles the case where someone just types "type"
    }
    let target = args[0];
    //now let's check if target is one of our built in commands
    match target {
        "exit" | "echo" | "type" | "bye" => {
            println!("{} is a shell builtin", target)
        }

        _ => {
        if let Ok(path_env) = std::env::var("PATH"){
            for directory in path_env.split(':'){
               let mut path = std::path::PathBuf::from(directory);
                path.push(target);
               
                 if path.exists() {
                    if let Ok(metadata) = std::fs::metadata(&path){
                        let mode = metadata.permissions().mode();
                        if mode & 0o111 != 0 {
                            println! ("{} is {}",target, path.display());
                            return;
                        }
                    }
            }
        }
       }  
        println!("{}: not found", target)
    }
    }
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
        "type" => handle_type(args),
        _ => println!("{}: command not found", command)
    }

}
       
        
    }


    