use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Keyword(String),
    BinaryOp(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    ListData(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "void"),
            Object::Keyword(s) => write!(f, "{}", s),
            Object::BinaryOp(s) => write!(f, "{}", s),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::ListData(list) => {
                write!(f, "(");
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", obj);
                }
                write!(f, ")")
            }
            _ => write!(f, "something cant be displayed"),
        }
    }
}
