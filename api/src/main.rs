use actix_web::{App, HttpServer};
mod routes;

fn main() {
    HttpServer::new(|| {
        App::new()
        .configure(routes::user::user_scoped_config)
    })  //can set #workers here. default value is #logical cores
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
