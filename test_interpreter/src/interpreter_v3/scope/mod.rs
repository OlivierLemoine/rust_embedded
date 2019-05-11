mod code_list;
mod flags;
mod var;

use code_list::CodeList;
use flags::Flags;
use var::Var;

pub struct Scope<'a> {
    code: Vec<&'a str>,
    flags: Flags,

    vars: Vec<Var<'a>>,
    acc: Var<'a>,

    instruction_pointer: usize,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<&str>) -> Scope {
        Scope {
            code,
            flags: Flags::new(),

            vars: Vec::new(),
            acc: Var::new(),

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
            CodeList::Def => {}
            CodeList::Call => {}
        }

        self.instruction_pointer += 1;
    }

    fn decode(&self) -> (CodeList, Vec<&Var>) {
        let line = self.code[self.instruction_pointer];

        if line.starts_with("//") || line.is_empty() {
            return (CodeList::Null, Vec::new());
        }

        (CodeList::Null, Vec::new())
    }
}
