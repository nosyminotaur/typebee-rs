#[macro_use]
use crate::schema::users;
use argon2::Config;
use rand::Rng; 
use rand::distributions::Alphanumeric;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    created_at: SystemTime,
    last_login: Option<SystemTime>
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    username: String,
    passhash: String,
    email: String,
    created_at: SystemTime,
    last_login: Option<SystemTime>
}

impl NewUser {
    //creates a password hash using a random salt and then returns a NewUser
    pub fn new<S: Into<String>>(username: S, password: S, email: S) -> NewUser {
        let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();
        let passhash = &argon2::hash_encoded(password.into().as_bytes(), salt.as_bytes(), &Config::default()).unwrap();
        NewUser {
            username: username.into(),
            passhash: passhash.into(),
            email: email.into(),
            created_at: SystemTime::now(),
            last_login: Some(SystemTime::now())
        }
    }
}