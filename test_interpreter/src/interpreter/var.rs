pub enum VarType {
    None,
    Number,
    String,
    Bool,
    Array,
    Function,
}

pub struct Var {
    pub name: String,
    var_type: VarType,
    num_value: f32,
    str_value: String,
    bool_value: bool,
    arr_value: Vec<Var>,
    pub line_def: usize,
}

impl Var {
    pub fn new(name: &str, at: usize) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::None,
            num_value: 0.0,
            str_value: String::new(),
            bool_value: false,
            arr_value: Vec::new(),
            line_def: at,
        }
    }

    pub fn clone(&self) -> Var {
        let mut arr_value: Vec<Var> = Vec::new();

        for i in self.arr_value.iter() {
            arr_value.push(i.clone());
        }

        let var_type = match self.var_type {
            VarType::None => VarType::None,
            VarType::Number => VarType::Number,
            VarType::String => VarType::String,
            VarType::Bool => VarType::Bool,
            VarType::Array => VarType::Array,
            VarType::Function => VarType::Function,
        };

        Var {
            name: self.name.clone(),
            var_type,
            num_value: self.num_value,
            str_value: self.str_value.clone(),
            bool_value: self.bool_value,
            arr_value,
            line_def: self.line_def,
        }
    }

    pub fn clone_and_rename(&self, name: &str) -> Var {
        let mut arr_value: Vec<Var> = Vec::new();

        for i in self.arr_value.iter() {
            arr_value.push(i.clone());
        }

        let var_type = match self.var_type {
            VarType::None => VarType::None,
            VarType::Number => VarType::Number,
            VarType::String => VarType::String,
            VarType::Bool => VarType::Bool,
            VarType::Array => VarType::Array,
            VarType::Function => VarType::Function,
        };

        Var {
            name: String::from(name),
            var_type,
            num_value: self.num_value,
            str_value: self.str_value.clone(),
            bool_value: self.bool_value,
            arr_value,
            line_def: self.line_def,
        }
    }

    pub fn number(name: &str, value: f32) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::Number,
            num_value: value,
            str_value: String::new(),
            bool_value: false,
            arr_value: Vec::new(),
            line_def: 0,
        }
    }

    pub fn string(name: &str, value: &str) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::String,
            num_value: 0.0,
            str_value: String::from(value),
            bool_value: false,
            arr_value: Vec::new(),
            line_def: 0,
        }
    }

    pub fn array(name: &str, value: Vec<Var>) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::Array,
            num_value: 0.0,
            str_value: String::new(),
            bool_value: false,
            arr_value: value,
            line_def: 0,
        }
    }

    pub fn function(name: &str, at: usize) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::Function,
            num_value: 0.0,
            str_value: String::new(),
            bool_value: false,
            arr_value: Vec::new(),
            line_def: at,
        }
    }

    pub fn boolean(name: &str, value: bool) -> Var {
        Var {
            name: String::from(name),
            var_type: VarType::Bool,
            num_value: 0.0,
            str_value: String::new(),
            bool_value: value,
            arr_value: Vec::new(),
            line_def: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        match self.var_type {
            VarType::None => true,
            _ => false,
        }
    }

    pub fn get_number(&self) -> f32 {
        match self.var_type {
            VarType::None => 0.0,
            VarType::Number => self.num_value,
            VarType::String => self
                .str_value
                .parse()
                .expect("This string can't be converted into a number"),
            VarType::Bool => {
                if self.bool_value {
                    1.0
                } else {
                    0.0
                }
            }
            VarType::Array => panic!("An array can't be converted into a number"),
            VarType::Function => panic!("A function can't be converted into a number"),
        }
    }

    pub fn get_string(&self) -> String {
        match self.var_type {
            VarType::None => String::from(""),
            VarType::Number => self.num_value.to_string(),
            VarType::String => self.str_value.clone(),
            VarType::Bool => {
                if self.bool_value {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
            VarType::Array => {
                let mut res = String::from("[");

                for i in self.arr_value.iter() {
                    let s = i.get_string();
                    res.push_str(s.as_str());
                    res.push_str(", ");
                }
                if res.len() > 3 {
                    res.remove(res.len() - 1);
                    res.remove(res.len() - 1);
                }
                res.push(']');

                res
            }
            VarType::Function => format!("{} at {}", self.name, self.line_def),
        }
    }

    pub fn get_bool(&self) -> bool {
        match self.var_type {
            VarType::None => false,
            VarType::Number => !(self.num_value == 0.0),
            VarType::String => match self.str_value.as_str() {
                "true" => true,
                _ => false,
            },
            VarType::Bool => self.bool_value,
            VarType::Array => panic!("A function can't be converted into a boolean"),
            VarType::Function => panic!("A function can't be converted into a boolean"),
        }
    }

    pub fn get_array(&self) -> Vec<Var> {
        match self.var_type {
            VarType::None => Vec::with_capacity(1),
            VarType::Number => vec![self.clone()],
            VarType::String => {
                let mut res: Vec<Var> = Vec::new();

                for i in self.str_value.chars() {
                    let mut s = String::new();
                    s.push(i);
                    res.push(Var::string("", s.as_str()));
                }

                res
            }
            VarType::Bool => vec![self.clone()],
            VarType::Array => {
                let mut res: Vec<Var> = Vec::new();

                for i in &self.arr_value {
                    res.push(i.clone());
                }

                res
            }
            VarType::Function => panic!("A function can't be converted into an array"),
        }
    }

    pub fn push(&mut self, v: Var) {
        self.arr_value.push(v);
    }

    pub fn pop(&mut self) -> Var {
        match self.arr_value.pop() {
            Some(v) => v,
            None => Var::new("", 0),
        }
    }
}
