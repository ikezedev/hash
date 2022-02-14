use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HasuraUtilsError {
    Other(&'static str),
    Parse(serde_json::Error),
    Request(reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct OtherError(pub &'static str);

impl Display for HasuraUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HasuraUtilsError::Other(err) => write!(f, "{err}"),
            HasuraUtilsError::Parse(parse_error) => parse_error.fmt(f),
            HasuraUtilsError::Request(request_error) => request_error.fmt(f),
        }
    }
}

impl Error for HasuraUtilsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            HasuraUtilsError::Other(_) => None,
            HasuraUtilsError::Parse(ref e) => Some(e),
            HasuraUtilsError::Request(ref e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for HasuraUtilsError {
    fn from(err: serde_json::Error) -> Self {
        HasuraUtilsError::Parse(err)
    }
}

impl From<reqwest::Error> for HasuraUtilsError {
    fn from(err: reqwest::Error) -> Self {
        HasuraUtilsError::Request(err)
    }
}

impl From<OtherError> for HasuraUtilsError {
    fn from(err: OtherError) -> Self {
        HasuraUtilsError::Other(err.0)
    }
}
