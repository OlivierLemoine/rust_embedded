pub struct Scope<'a> {
    code: Vec<&'a str>,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<&str>) -> Scope {
        Scope { code }
    }

    pub fn run(&self) {}
}
