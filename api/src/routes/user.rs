extern crate actix_web;
use actix_web::{web, HttpRequest, Responder, Error, guard};
use common::{ ApiResponse, Response};
use common::errors::ApiError;
use common::models::user::{LoginRequest, LoginResponse, UserClaims};
use futures::future::{ok, Future};
// use jsonwebtoken::{Header, encode};

fn login(user_info: web::Json<LoginRequest>, _req: HttpRequest) -> Box<dyn Future<Item = impl Responder, Error = Error>> {
    let claims = UserClaims {
        email: user_info.email.clone(),
        username: user_info.username.clone()
    };

    Box::new(ok::<ApiResponse<LoginResponse>, Error>(
        LoginResponse::from_error(ApiError::InternalServerError)
    ))
}

//add all user paths here
//this can be added to the Application Factory by `configure()` method
//this config can contain it's own data, services, routes
pub fn user_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .route("/login", web::post().to_async(login).guard(guard::Header("Content-Type", "application/json"))) //only allow json requests
    );
}