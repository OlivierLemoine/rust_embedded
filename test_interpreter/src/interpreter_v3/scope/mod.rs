mod var;
use var::Var;

pub struct Scope<'a> {
    code: Vec<&'a str>,
    vars: Vec<Var<'a>>,

    instruction_pointer: usize,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<&str>) -> Scope {
        Scope {
            code,
            vars: Vec::new(),

            instruction_pointer: 0,
        }
    }

    pub fn run(&self) {}
}
