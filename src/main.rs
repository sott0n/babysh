use std::io::{stdin, stdout, Write};
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

        let mut child = Command::new(cmd).args(args).spawn().unwrap();

        let _ = child.wait();
    }
}
