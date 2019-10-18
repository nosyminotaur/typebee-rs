use serde::{ Serialize, Serializer, ser::SerializeStruct };
use actix_web::{ http, dev::HttpResponseBuilder, HttpResponse, Responder, HttpRequest };
pub mod models;
pub mod errors;
use errors::ApiError;

//remove ?Sized to know why it was inherited xD
pub struct ApiResponse<T: Response + ?Sized> {
    success: bool,
    error: ApiError,
    status: Status,
    response: T,
}

pub trait Response: Serialize {
    //creates a new response with success as false
    //Serialze added because serde is used to convert to json
    fn from_error(error: ApiError) -> ApiResponse<Self>;
    //returns an empty Object that can be returned safely for failures
    fn default() -> Self;
}

impl<T> Responder for ApiResponse<T> where T: Response {
    //No need of using ApiError here because it is already present in
    //Self::Future or HttpResponse due to to_response()
    type Error = actix_web::Error;
    type Future = Result<HttpResponse, Self::Error>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        self.to_response()
    }
}

impl<T> ApiResponse<T> where T: Response {
    pub fn to_response(&self) -> Result<HttpResponse,actix_web:: Error> {
        let body = serde_json::to_string(&self)?;
        //now builder has the status code present in response.response_type
        let mut builder = self.status.response_type.to_response_builder();
        let response = builder.content_type("application/json").body(body);
        Ok(response)
    }

    pub fn new(response: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            error: ApiError::NoError,
            status: Status::new(StatusCode::Ok),
            response
        }
    }

    pub fn with_status(response: T, status: Status) -> ApiResponse<T> {
       ApiResponse {
            success: true,
            error: ApiError::NoError,
            status,
            response
        }
    }

    pub fn from_err(error: ApiError) -> ApiResponse<T> {
        let status = match error {
            ApiError::InternalServerError => Status::new(StatusCode::InternalServerError),
            _ => Status::new(StatusCode::BadRequest)
        };

        ApiResponse {
            success: false,
            error,
            status,
            response: T::default()
        }
    }
}

impl<T> Serialize for ApiResponse<T> where T: Response + ?Sized {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("ApiResponse", 4)?;
        state.serialize_field("success", &self.success)?;
        state.serialize_field("status", &self.status)?;
        if self.success {
            state.serialize_field("response", &self.response)?;
        }
        else {
            state.serialize_field("error", &self.error)?;
        }
        state.end()
    }
}


//ApiError doesn't hold any infomation on the Return Status Code.
//This information is always held by this struct
//It needs to be present in every response
//the fields of Response aren't public because
//the status_code field of Response must match the StatusCode enum
#[derive(Serialize)]
pub struct Status {
    response_type: StatusCode,
    status_code: u16
}

impl Status {
    //the need for this function is that we need to ensure that
    //the status_code field of Response matches the StatusCode enum
    pub fn new(response_type: StatusCode) -> Status {
        Status {
            response_type,
            status_code: StatusCode::to_status_code_int(response_type)
        }
    }
}

//centralized response type.
//more responses to be added depending on
//whether we need them
#[allow(dead_code)]
#[derive(Serialize, Clone, Copy)]
pub enum StatusCode {
    Ok,
    InternalServerError,
    BadRequest,
    Timeout,
}

impl StatusCode {
    //converts enum to status code for HttpResponse
    pub fn to_status_code(response_type: StatusCode) -> http::StatusCode {
        match response_type {
            StatusCode::Ok => http::StatusCode::OK,
            StatusCode::InternalServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BadRequest => http::StatusCode::BAD_REQUEST,
            StatusCode::Timeout => http::StatusCode::REQUEST_TIMEOUT,
        }
    }

    pub fn to_status_code_int(response_type: StatusCode) -> u16 {
        StatusCode::to_status_code(response_type).as_u16()
    }

    //returns an HttpResponseBuilder that can be used to create an HttpResponse
    pub fn to_response_builder(self) -> HttpResponseBuilder {
        HttpResponseBuilder::new(StatusCode::to_status_code(self))
    }
}