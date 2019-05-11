use super::Scope;

enum DataRep<'a> {
    None,
    U32(u32),
    F32(f32),
    Bool(bool),
    Str(Box<String>),
    Arr(Box<Vec<Var<'a>>>),
    Fun(Box<Scope<'a>>),
}

pub struct Var<'a> {
    data_raw: DataRep<'a>,
}

impl<'a> Var<'a> {
    pub fn new() -> Var<'a> {
        Var {
            data_raw: DataRep::U32(0),
        }
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
