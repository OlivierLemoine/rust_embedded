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
    let mut acc = String::new();
    while i < lines.len() {
        let keys: Vec<&str> = lines[i].trim().split(' ').collect();
        match keys[0] {
            "def" => {
                println!("add {}", keys[1]);
                ctx.vars.push(Var {
                    name: String::from(keys[1]),
                    value: String::from(keys[1]),
                    line: i + 1,
                });
                while lines[i].trim() != "end" {
                    i += 1;
                }
            }
            "call" => match ctx.find(keys[1]) {
                Some(v) => {
                    println!("call {}", keys[1]);
                    let mut vars: Vec<Var> = Vec::new();
                    for j in 2..keys.len() {
                        vars.push(Var {
                            name: String::new(),
                            value: String::new(),
                            line: 0,
                        })
                    }
                    let a = interp(
                        lines,
                        v.line,
                        Ctx {
                            vars,
                            parent: Some(&ctx),
                        },
                    );

                    println!("result : {}", a);
                }
                None => match keys[1] {
                    "add" => {
                        acc = String::from("0");
                    }
                    _ => panic!("No function named {}", keys[1]),
                },
            },
            ">" => {}
            "return" => {
                return acc;
            }
            "end" => {
                return String::new();
            }
            _ => {}
        }
        i += 1;
    }

    return String::new();
}

fn main() {
    let file_content = String::from_utf8(fs::read("./test").unwrap()).unwrap();
    let lines: Vec<&str> = file_content.split("\n").collect();
    interp(&lines, 0, Ctx::new());
    println!("end");
}
