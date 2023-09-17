use std::fmt::{Debug, Display};

use actix_web::{HttpResponse, ResponseError};

pub struct RequestError {
    original_error: reqwest::Error,
}

impl std::convert::From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            original_error: error,
        }
    }
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.original_error, f)
    }
}

impl Debug for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.original_error, f)
    }
}

impl std::error::Error for RequestError {}

impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}
