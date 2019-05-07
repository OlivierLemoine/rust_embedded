use std::fs;

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

fn interp(lines: &Vec<&str>, at: usize, mut ctx: Ctx) -> String {
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
        let keys: Vec<&str> = lines[i].trim().split(' ').collect();
        match keys[0] {
            "def" => {
                // println!("add {}", keys[1]);
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
                    3 => ctx.find(keys[2]).unwrap().value.clone(),
                    _ => acc.clone(),
                };
                return r;
            }
            "end" => {
                return String::new();
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

                    let a = interp(
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
                    "add" => {
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
                        acc = (r1 + r2).to_string();
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

    return String::new();
}

fn main() {
    let file_content = String::from_utf8(fs::read("./test").unwrap()).unwrap();
    let lines: Vec<&str> = file_content.split("\n").collect();
    interp(&lines, 0, Ctx::new());
}
