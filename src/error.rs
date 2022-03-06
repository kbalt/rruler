use std::fmt;

pub type IResult<I, O> = nom::IResult<I, O, ParseError>;

pub struct ParseError {
    errors: Vec<String>,
}

impl ParseError {
    pub(crate) fn from_tz_parse(msg: String) -> Self {
        Self { errors: vec![msg] }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.errors)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.errors.iter();

        let first = iter.next().unwrap();

        writeln!(f, "{}", first)?;

        for others in iter {
            writeln!(f, "  {}", others)?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

impl nom::error::ParseError<&str> for ParseError {
    fn from_error_kind(i: &str, _kind: nom::error::ErrorKind) -> Self {
        Self {
            errors: vec![format!("failed to parse '{i}'")],
        }
    }

    fn append(i: &str, _kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.errors.push(format!("failed to parse '{i}'"));
        other
    }
}

impl nom::error::ContextError<&str> for ParseError {
    fn add_context(i: &str, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push(format!("'{i}' - {ctx}"));
        other
    }
}

impl<E> nom::error::FromExternalError<&str, E> for ParseError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from_external_error(i: &str, _: nom::error::ErrorKind, e: E) -> Self {
        Self {
            errors: vec![format!("failed to parse '{i}' - {e}")],
        }
    }
}
