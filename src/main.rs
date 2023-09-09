extern crate diesel;

use std::env;
use std::sync::Mutex;

use actix_web::{App, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger;

mod schema;
mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let connection = web::Data::new(Mutex::new(connection));

    let addr = format!("{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(connection.clone())
            .service(service::get_root)
            .service(service::get_users)
            .service(service::get_questions)
            .service(service::post_answers)
            .service(service::post_users)
    })
        .bind(&addr)?
        .run()
        .await
}
