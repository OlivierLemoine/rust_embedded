mod code_list;
mod flags;
mod var;

use code_list::CodeList;
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

        let (instruction, args) = self.decode();

        match instruction {
            CodeList::Null => {}
            CodeList::Attribution => {
                let a = args[0].clone();
                self.vars.push(a);
            }
            CodeList::Def => {}
            CodeList::Call => {}
        }

        self.instruction_pointer += 1;
    }

    fn decode(&mut self) -> (CodeList, Vec<&Var<'a>>) {
        let line = self.code[self.instruction_pointer];

        if line.starts_with("//") || line.is_empty() {
            return (CodeList::Null, Vec::new());
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
            // "$" => (CodeList::Def, {
            //         let tmp = self;
            //         // let a = self.parse_var();
            //         // vec![a]
            //     }),
            ">" => (CodeList::Attribution, {
                self.acc.rename(words[1]);
                vec![&self.acc]
            }),
            _ => (CodeList::Null, Vec::new()),
        }
    }

    fn parse_var(&mut self) -> &Var {
        let res = self.find_var();
        res
    }

    fn find_var(&self) -> &Var {
        &self.vars[0]
    }

    fn induce_var_val(&self) -> Var {
        Var::new(None)
    }
}
