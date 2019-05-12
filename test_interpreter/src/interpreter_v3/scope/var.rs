use super::Scope;

#[derive(Clone)]
enum DataRep<'a> {
    None,
    I32(i32),
    F32(f32),
    Bool(bool),
    Str(Box<String>),
    Arr(Box<Vec<Var<'a>>>),
    Fun(Box<Scope<'a>>),
}

#[derive(Clone)]
pub struct Var<'a> {
    name: Option<&'a str>,
    data_raw: DataRep<'a>,
}

impl<'a> Var<'a> {
    pub fn new(name: Option<&'a str>) -> Var<'a> {
        Var {
            name,
            data_raw: DataRep::I32(0),
        }
    }

    pub fn match_name(&self, value: &str) -> bool {
        match self.name {
            Some(v) => v == value,
            None => false,
        }
    }

    pub fn integer(&mut self, value: i32) {
        self.data_raw = DataRep::I32(value);
    }

    pub fn float(&mut self, value: f32) {
        self.data_raw = DataRep::F32(value);
    }

    pub fn boolean(&mut self, value: bool) {
        self.data_raw = DataRep::Bool(value);
    }

    pub fn string(&mut self, value: Box<String>) {
        self.data_raw = DataRep::Str(value);
    }

    pub fn array(&mut self, value: Box<Vec<Var<'a>>>) {
        self.data_raw = DataRep::Arr(value);
    }

    pub fn function(&mut self, value: Box<Scope<'a>>) {
        self.data_raw = DataRep::Fun(value);
    }

    pub fn rename(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    pub fn get_integer(&self) -> Result<i32, &str> {
        let a = &self.data_raw;
        match &self.data_raw {
            DataRep::None => Ok(0),
            DataRep::I32(v) => Ok(*v),
            DataRep::F32(v) => Ok(*v as i32),
            DataRep::Bool(_) => Ok(1),
            DataRep::Str(_) => Err("A Strintcannot be converted into an integer"),
            DataRep::Arr(_) => Err("An Array cannot be converted into an integer"),
            DataRep::Fun(_) => Err("A Function cannot be converted into an integer"),
        }
    }
}
