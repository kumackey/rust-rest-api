extern crate diesel;

use std::env;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::sync::Mutex;

use schema::users::dsl::*;

#[get("/")]
async fn hello(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match query(db).await {
        Ok(s) => HttpResponse::Ok().body(s),
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let connection = web::Data::new(Mutex::new(connection));

    let addr = format!("{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(&addr)?
        .run()
        .await
}

async fn query(db: web::Data<Mutex<PgConnection>>) -> Result<String, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let results = users
        .limit(5)
        .load::<User>(&mut *conn)
        .expect("Error loading users");

    Ok(format!("users: {}", results.len()))
}

mod schema;

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
}