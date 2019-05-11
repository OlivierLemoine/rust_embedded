mod debugger;
mod scope;

pub struct Interpreter<'a> {
    lines: Vec<&'a str>,
    debugger: Option<debugger::Debugger>,
}

impl<'a> Interpreter<'a> {
    pub fn new(file: &str) -> Interpreter {
        Interpreter {
            lines: file.split("\n").map(|s| s.trim()).collect(),
            debugger: None,
        }
    }

    pub fn start(&self) {
        let global_scope = scope::Scope::new(self.lines[..].to_vec());
        global_scope.run();
    }
}
