use std::fs;
use std::{thread, time};

enum Msg {
    None,
    Break,
    Return,
    Continue,
}

struct Var {
    name: String,
    value: String,
    line: usize,
}

struct Ctx<'a> {
    vars: Vec<Var>,
    parent: Option<&'a Ctx<'a>>,
}

impl<'a> Ctx<'a> {
    pub fn new() -> Ctx<'a> {
        Ctx {
            vars: Vec::new(),
            parent: None,
        }
    }

    pub fn find(&self, name: &str) -> Option<&Var> {
        match self.vars.iter().find(|a: &&Var| a.name == name) {
            Some(v) => Some(v),
            None => match self.parent {
                Some(p) => p.find(name),
                None => None,
            },
        }
    }
}

fn interp(lines: &Vec<&str>, at: usize, mut ctx: Ctx) -> (String, Msg) {
    let mut i = at;

    match ctx.parent {
        Some(_) => {
            let names: Vec<&str> = lines[i].trim().split(' ').collect();

            for j in 2..names.len() {
                ctx.vars[j - 2].name = String::from(names[j]);
            }

            i += 1;
        }
        None => {}
    };

    let mut acc = String::new();
    while i < lines.len() {
        // println!("{} : {}", i, lines[i]);
        // thread::sleep(time::Duration::from_millis(2000));
        if lines[i].trim().starts_with("//") {
            continue;
        }
        let keys: Vec<&str> = lines[i].trim().split(' ').collect();
        match keys[0] {
            "def" => {
                ctx.vars.push(Var {
                    name: String::from(keys[1]),
                    value: String::from(keys[1]),
                    line: i,
                });
                while lines[i].trim() != "end" {
                    i += 1;
                }
            }
            ">" => ctx.vars.push(Var {
                name: String::from(keys[1]),
                value: acc.clone(),
                line: 0,
            }),
            "return" => {
                let r = match keys.len() {
                    2 => ctx.find(keys[1]).unwrap().value.clone(),
                    _ => acc.clone(),
                };
                return (r, Msg::Return);
            }
            "end" => {
                return (acc, Msg::None);
            }
            "else" => {
                let mut depth = 0;
                while lines[i].trim() != "end" || depth != 0 {
                    if lines[i].trim().starts_with("if") {
                        depth += 1;
                    }
                    if lines[i].trim().starts_with("endif") {
                        depth -= 1;
                    }
                    i += 1;
                }
            }
            "endif" => {}
            "if" => {
                let boolean: f32 = match keys.len() {
                    2 => match keys[1].parse() {
                        Ok(v) => v,
                        Err(_) => ctx.find(keys[1]).unwrap().value.parse().unwrap(),
                    },
                    _ => acc.parse().unwrap(),
                };

                if boolean != 0.0 {
                } else {
                    let mut depth = 0;
                    i += 1;
                    while lines[i].trim() != "else" && lines[i].trim() != "endif" || depth != 0 {
                        if lines[i].trim().starts_with("if") {
                            depth += 1;
                        }
                        if lines[i].trim().starts_with("endif") {
                            depth -= 1;
                        }
                        i += 1;
                    }
                }
            }
            x => match ctx.find(x) {
                Some(v) => {
                    let mut vars: Vec<Var> = Vec::new();
                    for j in 1..keys.len() {
                        vars.push(Var {
                            name: String::new(),
                            value: match ctx.find(keys[j]) {
                                Some(v) => v.value.clone(),
                                None => String::from(keys[j]),
                            },
                            line: 0,
                        })
                    }

                    vars.push(Var {
                        name: String::new(),
                        value: acc.clone(),
                        line: 0,
                    });

                    let (a, _) = interp(
                        lines,
                        v.line,
                        Ctx {
                            vars,
                            parent: Some(&ctx),
                        },
                    );

                    acc = a;
                }
                None => match x {
                    func @ "add" | func @ "sub" | func @ "mul" | func @ "div" | func @ "eq" => {
                        let r1: f32 = match keys.len() {
                            2 | 3 => match keys[1].parse() {
                                Ok(v) => v,
                                Err(_) => {
                                    let f = ctx.find(keys[1]).unwrap();
                                    let v = f.value.parse().unwrap();
                                    v
                                }
                            },
                            _ => {
                                let tmp: f32 = acc.parse().unwrap();
                                tmp
                            }
                        };
                        let r2: f32 = match keys.len() {
                            3 => match keys[2].parse() {
                                Ok(v) => v,
                                Err(_) => {
                                    let f = ctx.find(keys[2]).unwrap();
                                    let v = f.value.parse().unwrap();
                                    v
                                }
                            },
                            _ => {
                                let tmp: f32 = acc.parse().unwrap();
                                tmp
                            }
                        };
                        acc = match func {
                            "add" => r1 + r2,
                            "sub" => r1 - r2,
                            "mul" => r1 * r2,
                            "div" => r1 / r2,
                            "eq" => {
                                if r1 == r2 {
                                    1.0
                                } else {
                                    0.0
                                }
                            }
                            _ => 0.0,
                        }
                        .to_string();
                    }
                    func @ "$" => {
                        let r1: f32 = match keys.len() {
                            2 => match keys[1].parse() {
                                Ok(v) => v,
                                Err(_) => ctx.find(keys[1]).unwrap().value.parse().unwrap(),
                            },
                            _ => acc.parse().unwrap(),
                        };

                        acc = match func {
                            "$" => r1,
                            _ => 0.0,
                        }
                        .to_string();
                    }
                    "print" => {
                        let r = match keys.len() {
                            3 => ctx.find(keys[1]).unwrap().value.clone(),
                            _ => acc.clone(),
                        };
                        println!("{}", r);
                    }
                    _ => {}
                },
            },
        }
        i += 1;
    }

    return (acc, Msg::None);
}

fn main() {
    let file_content = String::from_utf8(fs::read("./test").unwrap()).unwrap();
    let lines: Vec<&str> = file_content.split("\n").collect();
    interp(&lines, 0, Ctx::new());
}
