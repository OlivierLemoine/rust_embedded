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

        if line.starts_with("//") || line == "" {
            i += 1;
            continue;
        }

        let words: Vec<&str> = line.split(' ').collect();

        let key_word = words[0];

        match key_word {
            "def" => {
                if words.len() < 2 {
                    panic!("{} : Missing function name", i + 1);
                }
                ctx.vars.push(Var::function(words[1], i));

                let mut depth = 0;

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
            ">" => {
                if words.len() < 2 {
                    panic!("{} : Missing variable name", i + 1);
                }

                if !ctx.replace(words[1], acc.clone()) {
                    ctx.vars.push(acc.clone());
                }
            }
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
            "end" => {
                return (acc, Msg::None);
            }
            "call" => match ctx.find(words[1]) {
                Some(v) => {
                    let mut vars: Vec<Var> = Vec::new();
                    for j in 2..words.len() {
                        vars.push(get_value(words[j], &ctx, i));
                    }

                    vars.push(acc.clone());

                    let (res, _) = run(
                        lines,
                        v.line_def,
                        Ctx {
                            vars,
                            parent: Some(&ctx),
                        },
                    );

                    acc = res;
                }
                None => panic!("{} : Unknown function {}", i + 1, words[1]),
            },
            "if" => {}
            "else" => {}
            "endif" => {}
            x => match ctx.find(x) {
                Some(v) => {
                    let mut vars: Vec<Var> = Vec::new();
                    for j in 1..words.len() {
                        vars.push(get_value(words[j], &ctx, i));
                    }

                    vars.push(acc.clone());

                    let (res, _) = run(
                        lines,
                        v.line_def,
                        Ctx {
                            vars,
                            parent: Some(&ctx),
                        },
                    );

                    acc = res;
                }
                None => match x {
                    func @ "add" | func @ "sub" | func @ "mul" | func @ "div" | func @ "eq" => {
                        let r1: Var = if words.len() > 1 {
                            get_value(words[1], &ctx, i)
                        } else {
                            acc.clone()
                        };

                        let r2: Var = if words.len() > 2 {
                            get_value(words[2], &ctx, i)
                        } else {
                            acc.clone()
                        };

                        acc = match func {
                            "eq" => Var::boolean("", r1.get_number() == r2.get_number()),
                            x => Var::number(
                                "",
                                match x {
                                    "add" => r1.get_number() + r2.get_number(),
                                    "sub" => r1.get_number() - r2.get_number(),
                                    "mul" => r1.get_number() * r2.get_number(),
                                    "div" => r1.get_number() / r2.get_number(),
                                    _ => 0.0,
                                },
                            ),
                        }
                    }
                    func @ "print" => {
                        let r1: Var = if words.len() > 1 {
                            get_value(words[1], &ctx, i)
                        } else {
                            acc.clone()
                        };

                        match func {
                            "print" => println!("{}", r1.get_string()),
                            _ => {}
                        }
                    }
                    _ => panic!("{} : Unknown function {}", i + 1, x),
                },
            },
        }

        i += 1;
    }

    (Var::new("", 0), Msg::None)
}

fn get_value(value: &str, ctx: &Ctx, at: usize) -> Var {
    match parse_value(value) {
        Some(v) => v,
        None => match ctx.find(value) {
            Some(v) => v.clone(),
            None => panic!("{} : Unknown variable name {}", at + 1, value),
        },
    }
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
