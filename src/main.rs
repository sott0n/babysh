use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::str::SplitWhitespace;

pub enum Cmd<'a> {
    Cd(&'a Path),
    Exit,
    Other(String, SplitWhitespace<'a>),
}

pub fn parse<'a>(input: &'a str) -> Cmd<'a> {
    let mut parts = input.trim().split_whitespace();
    let cmd = parts.next().unwrap();
    let args = parts;

    let cmd = match cmd {
        "cd" => {
            // default is `/` as new directory.
            let new_dir = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            Cmd::Cd(root)
        }
        "exit" => Cmd::Exit,
        _ => Cmd::Other(cmd.to_owned(), args),
    };
    cmd
}

pub fn exec(cmd: Cmd) {
    match cmd {
        Cmd::Cd(root) => {
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
        }
        Cmd::Exit => return,
        Cmd::Other(cmd, args) => {
            match Command::new(&cmd).args(args).spawn() {
                Ok(mut child) => {
                    let _ = child.wait();
                }
                Err(_) => eprintln!("babysh: command not found: {}", cmd),
            };
        }
    }
}

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let cmd = parse(&input);
        exec(cmd);
    }
}
