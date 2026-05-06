#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;



fn find_in_path(target: &str) -> Option<String> {
    if let Ok(path_env) = std::env::var("PATH") {
        for directory in path_env.split(':') {
            let mut path = std::path::PathBuf::from(directory);
            path.push(target);

            if path.exists() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        return Some(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    None
}
//command handlers
fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));   
}

fn handle_type(args: &[&str]) {
    if args.is_empty() {
        println!("type is a shell builtin");
        return;
    }
    let target = args[0];
    match target {
        "exit" | "echo" | "type" | "bye" => {
            println!("{} is a shell builtin", target);
        }
        _ => {
            if let Some(path) = find_in_path(target) {
                println!("{} is {}", target, path);
            } else {
                println!("{}: not found", target);
            }
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
      _ => {
    if let Some(_path) = find_in_path(command) {
        // Just use the 'command' name. 
        // Since it's in the PATH, Command::new will find it 
        // and Arg #0 will be exactly what the tester expects!
        let mut child = Command::new(command) 
            .args(args)
            .spawn()
            .expect("failed to execute process");

        child.wait().expect("process wasn't running");
    } else {
        println!("{}: command not found", command);
    }
}
        }
    }


}  