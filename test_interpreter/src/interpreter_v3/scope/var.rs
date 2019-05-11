use super::Scope;

#[derive(Clone)]
enum DataRep<'a> {
    None,
    U32(u32),
    F32(f32),
    Bool(bool),
    Str(Box<String>),
    Arr(Box<Vec<Var<'a>>>),
    Fun(Box<Scope<'a>>),
}

// impl<'a> Clone for Vec<Var<'a>> {
//     fn clone(&self) -> Vec<Var<'a>> {}
// }

#[derive(Clone)]
pub struct Var<'a> {
    name: Option<&'a str>,
    data_raw: DataRep<'a>,
}

impl<'a> Var<'a> {
    pub fn new(name: Option<&'a str>) -> Var<'a> {
        Var {
            name,
            data_raw: DataRep::U32(0),
        }
    }

    pub fn rename(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    pub fn test(&self) {
        let a = &self.data_raw;
        match &self.data_raw {
            DataRep::None => {}
            DataRep::U32(v) => {}
            DataRep::F32(v) => {}
            DataRep::Bool(v) => {}
            DataRep::Str(v) => {}
            DataRep::Arr(v) => {}
            DataRep::Fun(v) => {}
        }
    }
}
