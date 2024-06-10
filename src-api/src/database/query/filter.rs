use core::fmt;

use super::{QueryValue, VecQueryValue};

#[derive(Debug)]
pub enum Filter {
    If(String, String, QueryValue),
    In(String, Vec<QueryValue>)
}

impl Filter {
    pub fn if_from(lhs: &str, cmp: &str, rhs: QueryValue) -> Self {
        Self::If(lhs.into(), cmp.into(), rhs)
    }

    pub fn in_from(lhs: &str, rhs: Vec<QueryValue>) -> Self {
        Self::In(lhs.into(), rhs)
    }
}

pub trait VecFilterDisplay {
    fn to_string(&self, fjoin: &Option<Vec<JoinFilter>>) -> String;
}

impl VecFilterDisplay for Vec<Filter> {
    fn to_string(&self, fjoin: &Option<Vec<JoinFilter>>) -> String {
        let mut ret = String::new();
        for ( i, filter ) in self.iter().enumerate() {
            ret += &format!("{}", filter);
            if i+1 != self.len() {
                if let Some(joins) = fjoin {
                    if let Some(fj) = joins.get(i) {
                        ret += &fj.to_string();
                    }
                    else {
                        ret += &JoinFilter::And.to_string();
                    }
                }
                else {
                    ret += &JoinFilter::And.to_string();
                }
            }
        }
        ret
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::If(l, o, r) => {
                f.write_fmt(format_args!("{} {} {}", l, o , r))
            },
            Self::In(l, r) => {
                f.write_fmt(format_args!("{} IN {}", l, r.vqv_get()))
            },
        }
    }
}

pub enum JoinFilter {
    And,
    Or
}

impl fmt::Display for JoinFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => f.write_fmt(format_args!(" AND ")),
            Self::Or => f.write_fmt(format_args!(" OR ")),
        }
    }
}

