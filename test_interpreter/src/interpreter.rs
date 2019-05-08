pub enum Msg {
    None,
    Break,
    Return,
    Continue,
}

pub struct Var {
    name: String,
    value: String,
    line: usize,
}

pub struct Ctx<'a> {
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

fn cp_new_ctx<'a>(line: &str, mut ctx: Ctx<'a>) -> Ctx<'a> {
    let names: Vec<&str> = line.trim().split(' ').collect();
    for i in 2..names.len() {
        ctx.vars[i - 2].name = String::from(names[i]);
    }
    ctx
}

pub fn run(lines: &Vec<&str>, at: usize, mut ctx: Ctx) -> (String, Msg) {
    let i = at;

    match ctx.parent {
        Some(_) => ctx = cp_new_ctx(lines[i], ctx),
        None => {}
    }

    let acc = Var{
        name: String::new(),
        value: String::new(),
        line: 0,
    };

    (String::new(), Msg::None)
}
