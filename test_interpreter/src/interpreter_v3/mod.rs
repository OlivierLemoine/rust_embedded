mod scope;
mod debugger;

pub struct Interpreter{
    scope: scope::Scope,
    debugger: Option<debugger::Debugger>,
}

impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter{

        }
    }
}