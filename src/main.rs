use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts;

        match cmd {
            "cd" => {
                // default is `/` as new directory.
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            _ => {
                match Command::new(cmd).args(args).spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(_) => eprintln!("babysh: command not found: {}", cmd),
                };
            }
        }
    }
}
