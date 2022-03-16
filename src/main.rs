use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{exit, Child, Command, Stdio};
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

pub fn exec(cmd: Cmd, prev_cmd: Option<Child>, is_next_cmd: bool) -> Option<Child> {
    let out: Option<Child> = match cmd {
        Cmd::Cd(root) => {
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
            None
        }
        Cmd::Exit => exit(0),
        Cmd::Other(cmd, args) => {
            let stdin = prev_cmd.map_or(Stdio::inherit(), |output: Child| {
                Stdio::from(output.stdout.unwrap())
            });
            let stdout = if is_next_cmd {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let output = Command::new(&cmd)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            match output {
                Ok(output) => Some(output),
                Err(_) => {
                    eprintln!("babysh: command not found: {}", cmd);
                    None
                }
            }
        }
    };

    out
}

pub fn pipe<'a>(inputs: &'a str) {
    let mut input_list = inputs.trim().split(" | ").peekable();
    let mut prev_cmd = None;

    while let Some(input) = input_list.next() {
        let cmd = parse(&input);
        let is_next_cmd = if input_list.peek().is_some() {
            true
        } else {
            false
        };
        let output = exec(cmd, prev_cmd, is_next_cmd);
        prev_cmd = output;
    }

    if let Some(mut fin_cmd) = prev_cmd {
        let _ = fin_cmd.wait();
    }
}

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        pipe(&input);
    }
}
