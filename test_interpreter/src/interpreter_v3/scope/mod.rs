mod flags;
mod var;

use flags::Flags;
use var::Var;

#[derive(Clone)]
pub struct Scope<'a> {
    code: Vec<&'a str>,
    flags: Flags,

    parent_scope: Option<&'a Scope<'a>>,
    vars: Vec<Var<'a>>,
    acc: Var<'a>,

    instruction_pointer: usize,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<&str>) -> Scope {
        Scope {
            code,
            flags: Flags::new(),

            parent_scope: None,
            vars: Vec::new(),
            acc: Var::new(None),

            instruction_pointer: 0,
        }
    }

    pub fn has_finished(&self) -> bool {
        self.flags.has_finished()
    }

    pub fn step(&mut self) {
        if self.instruction_pointer == self.code.len() {
            self.flags.finished();
            return;
        }

        self.decode();

        self.instruction_pointer += 1;
    }

    fn decode(&mut self) {
        let line = self.code[self.instruction_pointer];

        if line.starts_with("//") || line.is_empty() {
            return;
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

        match words[0] {
            "$" => {
                let tmp: Option<&Var<'a>> = self.find_var(words[1]);
                let a: Var<'a> = match tmp {
                    Some(v) => v.clone(),
                    None => induce_var_val(words[1]).unwrap(),
                };
                self.acc = a;
            }
            ">" => {
                let mut v = self.acc.clone();
                v.rename(words[1]);
                self.vars.push(v);
            }
            "print" => {
                let mut v = self.find_var(words[1]);
                match v {
                    Some(val) => println!("{}", val.get_string().unwrap()),
                    None => println!(
                        "{}",
                        induce_var_val(words[1]).unwrap().get_string().unwrap()
                    ),
                }
            }
            _ => {}
        }
    }

    fn find_var(&self, name: &str) -> Option<&Var<'a>> {
        match self.vars.iter().find(|a| a.match_name(name)) {
            Some(v) => {
                let a = v;
                Some(a)
            }
            None => match self.parent_scope {
                Some(p) => p.find_var(name),
                None => None,
            },
        }
    }
}

fn induce_var_val(value: &str) -> Result<Var, &str> {
    let mut res = Var::new(None);
    if value.starts_with("\"") {
        res.string(Box::new(String::from(value)));
    } else if value == "true" || value == "false" {
        res.boolean(value.parse().unwrap());
    } else {
        match value.parse::<i32>() {
            Ok(v) => res.integer(v),
            Err(_) => match value.parse::<f32>() {
                Ok(v) => res.float(v),
                Err(_) => return Err("Could not find a suitable type for this value"),
            },
        }
    }
    Ok(res)
}
