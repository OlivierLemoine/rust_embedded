pub mod ctx;
mod var;

use ctx::Ctx;
use var::Var;

pub static mut DEBUG: bool = false;

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
        let line: &str = lines[i].trim();

        if line.starts_with("//") || line == "" {
            i += 1;
            continue;
        }

        let words: Vec<&str> = if !line.contains("\"") {
            line.split(' ').collect()
        } else {
            let mut res: Vec<&str> = Vec::new();

            let mut in_string = false;
            let mut index = 0;

            let working_line: Vec<char> = line.chars().collect();

            for j in 0..line.len() {
                let c = working_line[j];

                if c == '"' && !in_string {
                    in_string = true;
                } else if c == '"' && in_string {
                    in_string = false;
                } else if c == ' ' && !in_string {
                    if index != j {
                        res.push(&line[index..j]);
                    }
                    index = j + 1;
                }
                if j == line.len() - 1 {
                    res.push(&line[index..j + 1]);
                }
            }

            res
        };

        if words.len() == 0 {
            panic!("Error in parsing, probably not your fault");
        }

        if unsafe { DEBUG } {
            println!("{} : {}", i + 1, line);
            let mut tmp = String::new();
            std::io::stdin().read_line(&mut tmp).unwrap();
            let mut input = tmp.split(" ");
            match input.next() {
                Some(v) => match v.trim() {
                    "s" => {}
                    "p" => {
                        println!(
                            "{}",
                            match input.next() {
                                Some(v) => match ctx.find(v.trim()) {
                                    Some(value) => value.get_string(),
                                    None => String::from("Unknown variable"),
                                },
                                None => acc.get_string(),
                            }
                        );
                        continue;
                    }
                    "" => {}
                    _ => continue,
                },
                None => {}
            }
        }

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

                    if new_l.starts_with("def") {
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
                    ctx.vars.push(acc.clone_and_rename(words[1]));
                }
            }
            "$" => {
                if words.len() < 2 {
                    panic!("{} : No value", i + 1);
                }

                acc = get_value(words[1], &ctx, i);
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
            "if" => {
                let b: bool = if words.len() > 1 {
                    get_value(words[1], &ctx, i)
                } else {
                    acc.clone()
                }
                .get_bool();

                if b {
                    let mut vars: Vec<Var> = Vec::new();

                    vars.push(acc.clone());

                    let (res, msg) = run(
                        lines,
                        i,
                        Ctx {
                            vars,
                            parent: Some(&ctx),
                        },
                    );

                    acc = res;

                    match msg {
                        Msg::Break | Msg::Continue | Msg::None => {}
                        Msg::Return => {
                            return (acc, Msg::None);
                        }
                    }

                    let mut depth = 0;
                    loop {
                        i += 1;

                        let new_l = lines[i].trim();

                        if new_l.starts_with("if") {
                            depth += 1;
                        }
                        if new_l == "endif" {
                            depth -= 1;
                        }
                        if depth < 0 {
                            break;
                        }
                    }
                } else {
                    let mut depth = 0;
                    loop {
                        i += 1;

                        let new_l = lines[i].trim();

                        if new_l.starts_with("if") {
                            depth += 1;
                        }
                        if new_l == "endif" {
                            depth -= 1;
                        }
                        if depth < 0 {
                            break;
                        }

                        if depth == 0 && new_l == "else" {
                            let mut vars: Vec<Var> = Vec::new();

                            vars.push(acc.clone());

                            let (res, msg) = run(
                                lines,
                                i,
                                Ctx {
                                    vars,
                                    parent: Some(&ctx),
                                },
                            );

                            acc = res;

                            match msg {
                                Msg::Break | Msg::Continue | Msg::None => {}
                                Msg::Return => {
                                    return (acc, Msg::None);
                                }
                            }
                        }
                    }
                }
            }
            "else" => {
                return (acc, Msg::None);
            }
            "endif" => {
                return (acc, Msg::None);
            }
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
                    func @ "at" | func @ "push" => {
                        let mut r1: Var = if words.len() > 2 {
                            get_value(words[1], &ctx, i)
                        } else {
                            acc.clone()
                        };
                        let r2: Var = if words.len() > 2 {
                            get_value(words[2], &ctx, i)
                        } else {
                            get_value(words[1], &ctx, i)
                        };

                        match func {
                            "at" => {
                                let v1 = r1.get_array();
                                let v2 = r2.get_number() as usize;

                                acc = if v2 < v1.len() {
                                    r1.get_array()[r2.get_number() as usize].clone()
                                } else {
                                    Var::new("", 0)
                                };
                            }
                            "push" => {
                                r1.push(r2);
                                acc = r1;
                            }
                            _ => {}
                        }
                    }
                    "len" => {
                        let r1 = if words.len() > 1 {
                            get_value(words[1], &ctx, i)
                        } else {
                            acc.clone()
                        };

                        acc = Var::number("", r1.get_array().len() as f32);
                    }
                    "arr" => {
                        let mut vars: Vec<Var> = Vec::new();

                        for j in 1..words.len() {
                            vars.push(get_value(words[j], &ctx, i));
                        }

                        acc = Var::array("", vars);
                    }
                    "concat" => {
                        let mut vars: Vec<Var> = Vec::new();

                        for j in 1..words.len() {
                            let mut tmp = get_value(words[j], &ctx, i).get_array();
                            vars.append(&mut tmp);
                        }

                        acc = Var::array("", vars);
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

    if v.starts_with("\"") {
        Some(Var::string("", &value[1..value.len() - 1]))
    } else if v == "true" {
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
