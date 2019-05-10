use std::fs;

pub mod interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "-d" {
            unsafe { interpreter::DEBUG = true };
        }
    }

    let file_content = String::from_utf8(fs::read("./test.nc").unwrap()).unwrap();
    let lines: Vec<&str> = file_content.split("\n").collect();
    interpreter::run(&lines, 0, interpreter::ctx::Ctx::new());
}
