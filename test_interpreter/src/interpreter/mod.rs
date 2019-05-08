pub mod ctx;
mod var;

use ctx::Ctx;
use var::Var;

pub enum Msg {
    None,
    Break,
    Return,
    Continue,
}

pub fn run(lines: &Vec<&str>, at: usize, mut ctx: Ctx) -> (Var, Msg) {
    let mut i = at;

    if !ctx.is_root() {
        ctx.cp_new_ctx(lines[i]);
        i += 1;
    }

    let mut acc = Var::new("", 0);

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("//") {
            i += 1;
            continue;
        }

        let words: Vec<&str> = line.split(' ').collect();

        let key_word = words[0];

        match key_word {
            "def" => {
                if words.len() < 2 {
                    panic!("Missing function name");
                }
                ctx.vars.push(Var::function(words[1], i));

                let mut depth = 0;

                // while lines[i].trim() != "end" || depth != 0 {
                //     i += 1;
                //     if lines[i].trim() == "def"{
                //         depth += 1;
                //     }
                //     else if lines[i].trim() == "def"
                // }

                loop {
                    i += 1;

                    let new_l = lines[i].trim();

                    if new_l == "def" {
                        depth += 1;
                    } else if new_l == "end" {
                        depth -= 1;
                        if depth < 0 {
                            break;
                        }
                    }
                }
            }
            ">" => {}
            "return" => {
                return (
                    if words.len() == 2 {
                        match parse_value(words[1]) {
                            Some(v) => v,
                            None => ctx.find(words[1]).unwrap().clone(),
                        }
                    } else {
                        acc
                    },
                    Msg::Return,
                );
            }
            "end" => {}
            _ => {}
        }

        i += 1;
    }

    (Var::new("", 0), Msg::None)
}

fn parse_value(value: &str) -> Option<Var> {
    let v = value.trim();

    if v == "true" {
        Some(Var::boolean("", true))
    } else if v == "false" {
        Some(Var::boolean("", false))
    } else {
        let f: Result<f32, _> = v.parse();
        match f {
            Ok(val) => Some(Var::number("", val)),
            Err(_) => None,
        }
    }
}
