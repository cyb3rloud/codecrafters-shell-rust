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
fn handle_echo(args: &[&str], mut output_file: Option<std::fs::File>) {
    let output_text = args.join(" ");
    //check if we were given a file to write to
        if let Some(mut file) = output_file {
            //We have a file. Use writeln! to write to the text into it
            writeln!(file, "{}", output_text).unwrap();
            } else {
                //No file was provided. Print the terminal normally.
                println!("{}", output_text);
            }

}

fn handle_type(args: &[&str]) {
    if args.is_empty() {
        println!("type is a shell builtin");
        return;
    }
    let target = args[0];
    match target {
        "exit" | "echo" | "type" | "bye" | "pwd" => {
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
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let mut args = &parts[1..];

        //1. Create an empty container for our output file
        let mut output_file = None;
        
        if let Some(index) = args.iter().position(|&arg| arg == ">" || arg == "1>") {
            // We found a redirect symbol!
            // The target file is the very next item in the array
            let target_file = args[index + 1];
            //shrink the args
            args = &args[..index];
            //2. Create the file and put it in out container 
            if let Ok(file) = std::fs::File::create(target_file) {
                output_file = Some(file);
            }

        }
        match command {
            "exit" | "bye" => break,
            "echo" => handle_echo(args, output_file),
            "type" => handle_type(args),
            "pwd" => handle_pwd(),
            "cd" => handle_cd(args),

            _ => {
                if let Some(_path) = find_in_path(command) {
                    let mut cmd = Command::new(command);
                    cmd.args(args);

                    // 2. If we caught a redirection file earlier, tell the command to use it!
                    if let Some(file) = output_file {
                        cmd.stdout(file);
                    }
                    // Just use the 'command' name.
                    // Since it's in the PATH, Command::new will find it
                    // and Arg #0 will be exactly what the tester expects!
                    let mut child = cmd.spawn().expect("failed to execute process");

                    child.wait().expect("process wasn't running");
                } else {
                    //command not found prints to the screen ignoring the file
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
//Implement PWD functionality
fn handle_pwd() {
    if let Ok(current_path) = std::env::current_dir() {
        println!("{}", current_path.display());
    }
}

//Implement cd functionality
fn handle_cd(args: &[&str]) {
    //Let's make sure the user actually typed a path
    if args.is_empty() {
        return;
    }

    //Let's take the destination path from the arguments
    let path = args[0];

    if path == "~" {
        // Step 1:Fetch the HOME variable safely and change directory
        if let Ok(home_path) = std::env::var("HOME") {
            if let Err(_) = std::env::set_current_dir(home_path) {
                println!("cd: ~: No such file or directory");
            }
        }
    } else {
        //Attempt to change the directory and catch any errors
        if let Err(_) = std::env::set_current_dir(path) {
            println!("cd: {}: No such file or directory", path);
        }
    }
}
