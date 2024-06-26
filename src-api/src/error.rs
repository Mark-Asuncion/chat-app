use core::fmt;

#[derive(Debug)]
pub enum ErrTypes {
    // 400
    MissingFields,
    InvalidValue,
    // 401
    MissingCredentials,
    BadCredentials,
    NotAuthenticated,
    CredentialsExists,
    // 404
    NotFound,
    // 500
    DatabaseInstance,
    InternalServer,
}

impl fmt::Display for ErrTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFields => f.write_str("Missing Fields"),
            Self::InvalidValue => f.write_str("Invalid Value"),
            Self::NotFound => f.write_str("Not Found"),
            Self::DatabaseInstance => f.write_str("Database Instance"),
            Self::MissingCredentials => f.write_str("Missing Credentials"),
            Self::CredentialsExists => f.write_str("Credentials Already Exists"),
            Self::BadCredentials => f.write_str("Bad Credentials"),
            Self::NotAuthenticated => f.write_str("User is Not Authenticated"),
            Self::InternalServer => f.write_str("Internal Server Error"),
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

    pub fn invalid_value() -> Self {
        Self {
            err_type: ErrTypes::InvalidValue,
            what: "Error parsing data".into()
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

    pub fn missing_credentials() -> Self {
        Self {
            err_type: ErrTypes::MissingCredentials,
            what: String::new()
        }
    }

    pub fn credentials_exists() -> Self {
        Self {
            err_type: ErrTypes::CredentialsExists,
            what: String::new()
        }
    }

    pub fn bad_credentials() -> Self {
        Self {
            err_type: ErrTypes::BadCredentials,
            what: String::new()
        }
    }

    pub fn not_authenticated() -> Self {
        Self {
            err_type: ErrTypes::NotAuthenticated,
            what: String::new()
        }
    }

    pub fn internal_server(what: &str) -> Self {
        Self {
            err_type: ErrTypes::InternalServer,
            what: what.into()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.what.is_empty() {
            return f.write_fmt(format_args!("{}",
                self.err_type));
        }
        f.write_fmt(format_args!("[{}]::{}",
            self.err_type, self.what))
    }
}
