use std::fmt;
use std::vec::Vec;
use std::default::Default;

#[derive(Debug)]
pub enum QueryValue {
    Varchar(String),
    Int(i16)
}

// impl QueryValue {
//     pub fn get_str(&self) -> Option<&str>  {
//         if let Self::Varchar(v) = self {
//             return Some(v.as_str());
//         }
//         None
//     }
//
//     pub fn get_int(&self) -> Option<i16> {
//         if let Self::Int(v) = self {
//             return Some(v.clone());
//         }
//         None
//     }
// }

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
    fn vqv_get_no_closing(&self) -> String;
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

    fn vqv_get_no_closing(&self) -> String {
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

pub struct QueryBuilder {
    is_where:   bool,
    q:          String
}

impl QueryBuilder {
    pub fn build(&self) -> String {
        self.q.clone() + ";"
    }

    pub fn new() -> Self {
        Self {
            is_where: false,
            q: Default::default()
        }
    }

    pub fn insert(&mut self, table: &str, keys: Vec<QueryValue>) -> &mut Self {
        assert_eq!(keys.is_empty(), false);
        self.q += &format!("INSERT INTO {} ", table);
        self.q += &keys.vqv_get();
        self.q += " ";
        self
    }

    pub fn values(&mut self, values: Vec<Vec<QueryValue>>) -> &mut Self {
        self.q += "VALUES ";
        for (i, v) in values.iter().enumerate() {
            self.q += &v.vqv_get();
            if i+1 != values.len() {
                self.q += ", ";
            }
        }
        self
    }

    pub fn update(&mut self, table: &str) -> &mut Self {
        self.q += format!("UPDATE {} ", table).as_str();
        self
    }

    pub fn set(&mut self, key: &str, value: QueryValue) -> &mut Self {
        self.q += format!("SET {} = {} ", key ,value).as_str();
        self
    }

    pub fn select(&mut self, columns: Option<Vec<QueryValue>>) -> &mut Self {
        self.q += "SELECT ";
        if let Some(cols) = columns {
            self.q += &cols.vqv_get_no_closing();
        }
        else {
            self.q += " *";
        }
        self.q += " ";
        self
    }

    pub fn from(&mut self, table: &str) -> &mut Self {
        self.q += format!("FROM {} ", table).as_str();
        self
    }

    pub fn filter(&mut self, key: &str, operator: &str, value: QueryValue) -> &mut Self {
        if !self.is_where {
            self.q += "WHERE ";
            self.is_where = true;
        }
        self.q += format!("{} {} {} ", key, operator, value).as_str();
        self
    }

    pub fn filter_in(&mut self, key: &str, values: Vec<QueryValue>) -> &mut Self {
        if !self.is_where {
            self.q += "WHERE ";
            self.is_where = true;
        }
        self.q += &format!("{} IN ", key);
        self.q += &values.vqv_get();
        self.q += " ";
        self
    }

    pub fn and(&mut self) -> &mut Self {
        self.q += "AND ";
        self
    }

    pub fn or(&mut self) -> &mut Self {
        self.q += "OR ";
        self
    }

    pub fn order_by(&mut self, expr: &str) -> &mut Self {
        self.q += format!("ORDER BY {} ", expr).as_str();
        self
    }
}

#[test]
fn t() {
    use QueryValue::{ Varchar, Int };
    let mut qb = QueryBuilder::new();
    qb.select(Some(vec![Varchar(String::from("hello")), Varchar(String::from("asdcn"))]))
        .from("table")
        .filter("asd", "<=", Int(129))
        .and()
        .filter_in("adqw", vec![Varchar("hello".into()), Int(11)])
        .or()
        .filter_in("adqw", vec![Varchar("hello".into()), Int(11)])
        .order_by("qdoj");
    println!("{}", qb.build());
}

#[test]
fn t2() {
    use QueryValue::{ Varchar };
    let mut qb = QueryBuilder::new();
    qb.update("table")
        .set("key", Varchar("value".into()));
    println!("{}", qb.build());
}

#[test]
fn t3() {
    use QueryValue::{ Varchar, Int };
    let mut qb = QueryBuilder::new();
    let keys = vec![Varchar(String::from("hello")), Varchar(String::from("asdcn"))];
    let values = vec![
        vec![Varchar(String::from("hello")), Varchar(String::from("asdcn"))],
        vec![Int(32), Varchar(String::from("asdcn"))]
    ];
    qb.insert("table", keys)
        .values(values);
    println!("{}", qb.build());
}
