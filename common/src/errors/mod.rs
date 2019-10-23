use serde::{ Serialize, Serializer, ser::SerializeStruct };

pub mod user;

//centralized error type.
#[derive(Debug)]
pub enum ApiError {
    AuthError,
    InternalServerError,
    //to return no error (default response for no error)
    NoError
}

//TODO - to use inner error later on
impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Error", 4)?;
        let error = match self {
            ApiError::AuthError => "Authentication Error",
            ApiError::NoError => "No Error",
            _ => "Internal Server Error"
        };
        state.serialize_field("error", error)?;
        state.end()
    }
}