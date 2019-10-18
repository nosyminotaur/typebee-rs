use serde::{ Serialize };
use std::fmt;

pub mod db;
pub mod user;

//centralized error type.
#[derive(Serialize, Debug, Clone, Copy)]
pub enum ApiError {
    DbError,
    AuthError,
    //Must be used for handling all errors that shouldn't be sent
    //to the user
    InternalServerError
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //improve all the matches to use inner Error Object's description
        //instead of string
        match self {
            ApiError::AuthError => write!(f, "Authentication Error"),
            ApiError::DbError => write!(f, "Database Error"),
            ApiError::InternalServerError => write!(f, "Internal Server Error")
        }
    }
}

//return a 500 response because we don't want
impl From<serde_json::error::Error> for ApiError {
    fn from(_err: serde_json::error::Error) -> ApiError {
        ApiError::InternalServerError
    }
}