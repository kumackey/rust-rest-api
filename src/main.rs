extern crate diesel;

use std::env;
use std::sync::Mutex;

use env_logger;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use schema::users;
use schema::users::dsl::*;

mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
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
            .service(get_root)
            .service(get_users)
            .service(get_questions)
            .service(post_answers)
            .service(post_users)
    })
        .bind(&addr)?
        .run()
        .await
}

#[get("/")]
async fn get_root(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO /get_usersと同じことをしたいので、共通化してください
    match find_all_users(db).await {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

#[get("/users")]
async fn get_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match find_all_users(db).await {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

// 別にいらなかったけど、デバッグのために作った
#[post("/users")]
async fn post_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO: request bodyからnameを取得できるようにしてください・・・
    let nuser = NewUser {
        name: "test".to_string(),
    };

    let user = create_user(db, nuser).await;
    match user {
        Ok(user) => HttpResponse::Ok().body(
            serde_json::to_string(&user).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("post user failed"),
    }
}

#[get("/questions")]
async fn get_questions(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match find_all_users(db).await {
        // TODO: ここでuser_listを返してるけどquestionsを返すようにしたい
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

// TODO questions/:id/answersのGETを追加する
// 用途は、この質問に対する回答一覧を取得します

#[post("/answers")]
async fn post_answers(req_body: String) -> impl Responder {
    // TODO: questionを登録する
    // user_name, question_id, answerを受け取る
    // user_nameからusersテーブルをfindして、なかったら作る
    // user_idを取得する
    // answersテーブルはuser_id, question_id, answer

    HttpResponse::Ok().body("post answers")
}

// 以下データベース処理

async fn find_all_users(db: web::Data<Mutex<PgConnection>>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let results = users
        .limit(100)
        .load::<User>(&mut *conn)?;

    Ok(results)
}

// async fn find_by_name(db: web::Data<Mutex<PgConnection>>,name: String) -> Result<User, diesel::result::Error> {
//     let mut conn = db.lock().unwrap();
//
//     let result = users
//         .filter(name.eq(name))
//         .first::<User>(&mut *conn)?;
//
//     Ok(result)
// }

async fn create_user(db: web::Data<Mutex<PgConnection>>, user: NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let result = diesel::insert_into(users)
        .values(&user)
        .get_result(&mut *conn)?;

    Ok(result)
}

// async fn answer_question(db: web::Data<Mutex<PgConnection>>, answer: Answer) -> Result<Answer, diesel::result::Error> {
//     let mut conn = db.lock().unwrap();
//
//     let result = diesel::insert_into(answers)
//         .values(&answer)
//         .get_result(&mut *conn)?;
//
//     Ok(result)
// }

#[derive(Queryable, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

// #[derive(Queryable, Serialize, Deserialize, Insertable)]
// struct Question {
//     id: i32,
//     questioner_id: i32,
//     question: String,
//     answer: String,
// }
//
// #[derive(Queryable, Serialize, Deserialize, Insertable)]
// struct Answer {
//     id: i32,
//     question_id: i32,
//     answer: String,
//     answered_at: String,
// }
