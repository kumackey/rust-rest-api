extern crate diesel;

use std::env;
use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use schema::users::dsl::*;

#[get("/")]
async fn get_root(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO usersを呼び出したいので、get_usersの関数を使うようにする
    match find_all_users(db).await {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

#[get("/users")]
async fn get_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match find_all_users(db).await {
        // TODO: ここでusersを返すようにしたい
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

#[get("/questions")]
async fn get_questions(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match find_all_users(db).await {
        // TODO: ここでquestionsを返すようにしたい
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

// TODO questions/:id/answersのGETを追加する

#[post("/answers")]
async fn post_answers(req_body: String) -> impl Responder {
    // TODO: questionを登録する
    // user_name, question_id, answerを受け取る
    // user_nameからusersテーブルをfindして、なかったら作る
    // user_idを取得する
    // answersテーブルはuser_id, question_id, answer
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
            .service(get_root)
            .service(get_users)
            .service(get_questions)
            .service(post_answers)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(&addr)?
        .run()
        .await
}

async fn find_all_users(db: web::Data<Mutex<PgConnection>>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let results = users
        .limit(5)
        .load::<User>(&mut *conn)?;

    Ok(results)
}

mod schema;

#[derive(Queryable, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}