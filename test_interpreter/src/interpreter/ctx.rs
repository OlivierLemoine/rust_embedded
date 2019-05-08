use super::var::Var;

pub struct Ctx<'a> {
    pub vars: Vec<Var>,
    pub parent: Option<&'a Ctx<'a>>,
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

    pub fn cp_new_ctx(&mut self, line: &str) {
        let names: Vec<&str> = line.trim().split(' ').collect();
        for i in 2..names.len() {
            if i - 2 < self.vars.len(){

            self.vars[i - 2].name = String::from(names[i]);
            }
            else{
                self.vars.push(Var::new(names[i], 0));
            }
        }
    }

    pub fn is_root(&self) -> bool{
        match self.parent{
            Some(_) => false,
            None => true,
        }
    }
}
