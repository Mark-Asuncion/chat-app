use core::fmt;

#[derive(Debug)]
pub enum ErrTypes {
    MissingFields,
    InvalidValue,
    NotFound,
    DatabaseInstance
}
impl fmt::Display for ErrTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFields => f.write_fmt(format_args!("Missing Fields")),
            Self::InvalidValue => f.write_fmt(format_args!("Invalid Value")),
            Self::NotFound => f.write_fmt(format_args!("Not Found")),
            Self::DatabaseInstance => f.write_fmt(format_args!("Database Instance")),

        }
    }
}

#[derive(Debug)]
pub struct Error {
    err_type: ErrTypes,
    what: String
}

impl Error {
    pub fn new(err_type: ErrTypes, what: &str) -> Self {
        Self {
            err_type,
            what: what.into()
        }
    }

    pub fn not_found() -> Self {
        Self {
            err_type: ErrTypes::NotFound,
            what: "Information does not exist".into()
        }
    }

    pub fn acquire_instance() -> Self {
        Self {
            err_type: ErrTypes::DatabaseInstance,
            what: "Error occured acquiring instance".into()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{}]::{}",
            self.err_type, self.what))
    }
}
