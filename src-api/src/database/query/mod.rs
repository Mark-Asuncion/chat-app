use std::fmt;
use std::vec::Vec;

pub mod builder;
pub mod filter;

#[derive(Debug, Clone)]
pub enum QueryValue {
    Varchar(String),
    Int(i16)
}

#[derive(Debug)]
pub enum Operation {
    Select(String, Option<Vec<String>>),
    Update(String),
    Insert(String, Vec<String>)
}

#[derive(Debug)]
pub enum ORDER {
    ASC,
    DESC
}

impl fmt::Display for ORDER {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ASC => f.write_fmt(format_args!("ASC")),
            Self::DESC => f.write_fmt(format_args!("DESC")),
        }
    }
}

impl fmt::Display for QueryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryValue::Varchar(v) => f.write_fmt(format_args!("'{}'", v)),
            QueryValue::Int(v) => f.write_fmt(format_args!("{}", v))
        }
    }
}

pub trait VecQueryValue {
    fn vqv_get(&self) -> String;
    fn vqv_get_no_enclosing(&self) -> String;
}

impl VecQueryValue for Vec<QueryValue> {
    fn vqv_get(&self) -> String {
        let mut res = "(".to_string();
        for (i, v) in self.iter().enumerate() {
            res += &v.to_string();
            if i+1 != self.len() {
                res += ", ";
            }
        }
        res += ")";
        res
    }

    fn vqv_get_no_enclosing(&self) -> String {
        let mut res = String::new();
        for (i, v) in self.iter().enumerate() {
            res += &v.to_string();
            if i+1 != self.len() {
                res += ", ";
            }
        }
        res
    }
}
