use core::fmt;

type JoinOn = (String, String);
pub enum Join {
    // Left(String, JoinOn),
    Inner(String, JoinOn),
    // Right(String, JoinOn)
}

impl Join {
    // create other functions if needed
    pub fn inner(table: &str, on: (&str, &str)) -> Self {
        Self::Inner(table.into(), (on.0.into(), on.1.into()))
    }
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inner(lhs, on) => f.write_fmt(format_args!("INNER JOIN {} ON {} = {}", lhs, on.0, on.1))
        }
    }
}
