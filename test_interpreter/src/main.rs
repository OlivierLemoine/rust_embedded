use std::fs;

pub mod interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut debug = interpreter::DebugParams::new();

    if args.len() > 1 {
        if args[1] == "-d" {
            debug.is_on = true;
        }
    }

    let file_content = String::from_utf8(fs::read("./test.nsc").unwrap()).unwrap();
    let lines: Vec<&str> = file_content.split("\n").collect();
    interpreter::run(&lines, 0, interpreter::ctx::Ctx::new(), &mut debug);
}
