use regex::Error;

#[derive(Debug)]
pub enum RSTError {
    /// Regex compile error
    RegexError(regex::Error),
    /// Required element not found
    NotFound(&'static str),
}

impl std::error::Error for RSTError {}

impl From<regex::Error> for RSTError {
    /// From regex error
    fn from(e: Error) -> Self {
        Self::RegexError(e)
    }
}

impl std::fmt::Display for RSTError {
    /// fmt error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RSTError::NotFound(s) => write!(f, "required element missing: `{}`", s),
            RSTError::RegexError(e) => write!(f, "regex compile error: `{:?}`", e),
        }
    }
}