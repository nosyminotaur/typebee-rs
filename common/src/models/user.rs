//contains all models related to the user in any way
use serde::{ Serialize, Deserialize };
use crate::errors::ApiError;
use crate::{ ApiResponse, Response, StatusCode, Status };

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}

impl Response for LoginResponse {
   fn default() -> LoginResponse {
       LoginResponse {
           jwt: String::default()
       }
   }

   fn from_error(error: ApiError) -> ApiResponse<LoginResponse> {
       let status = match error {
            ApiError::InternalServerError => Status::new(StatusCode::InternalServerError),
            _ => Status::new(StatusCode::BadRequest)
        };

        ApiResponse {
            success: false,
            error,
            status,
            response: LoginResponse::default()
        }
   }
}

#[derive(Serialize)]
pub struct UserClaims {
    pub email: String,
    pub username: String
}