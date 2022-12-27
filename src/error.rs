use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use tera::Error;

impl IntoResponse for JSiteError {
    fn into_response(self) -> Response {
        println!("Error occurred on route: {}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal error, try again later".to_string(),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub enum JSiteError {
    PageNotFound(PathBuf),
    IOError(std::io::Error),
    TemplateError(tera::Error),
}

impl From<std::io::Error> for JSiteError {
    fn from(e: std::io::Error) -> Self {
        JSiteError::IOError(e)
    }
}

impl From<tera::Error> for JSiteError {
    fn from(e: Error) -> Self {
        Self::TemplateError(e)
    }
}

impl Display for JSiteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JSiteError::PageNotFound(e) => write!(f, "Page not found: {}", e.display()),
            JSiteError::IOError(e) => write!(f, "IO Error: {}", e),
            JSiteError::TemplateError(e) => write!(f, "Template rendering error: {}", e),
        }
    }
}

pub type PageResult<T> = Result<T, JSiteError>;
