#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate argon2;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
pub mod models;
mod schema;
use models::user::{ NewUser, User };
use common::errors::ApiError;

pub fn establish_connection() -> Option<PgConnection> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => return None
    };

    let connection = match PgConnection::establish(&database_url) {
        Ok(res) => res,
        Err(_) => return None
    };

    Some(connection)
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, email: &'a str, password: &'a str) -> Result<(), ApiError> {
    use crate::schema::users;
    let new_user = NewUser::new(username, password, email);

    let query_response = diesel::insert_into(users::table)
    .values(&new_user)
    .execute(conn);
    match query_response {
        Ok(res) => {
            println!("Success message: {}", res);
            Ok(())
        },
        Err(err_msg) => {
            println!("{}", err_msg);
            Err(ApiError::InternalServerError)
        }
    }
}