mod debugger;
mod scope;

pub struct Interpreter<'a> {
    lines: Vec<&'a str>,
    scope: scope::Scope,
    debugger: Option<debugger::Debugger>,
}

impl<'a> Interpreter<'a> {
    pub fn new(file: &str) -> Interpreter {
        Interpreter {
            lines: file.split("\n").map(|s| s.trim()).collect(),
            scope: scope::Scope::new(),
            debugger: None,
        }
    }

    pub fn start(&self){
        
    }
}
