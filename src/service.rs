use std::sync::Mutex;

use actix_web::{get, HttpResponse, post, Responder, web};
use diesel::pg::PgConnection;

use crate::model;

#[get("/")]
pub async fn get_root(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO /get_usersと同じことをしたいので、共通化してください
    let mut conn = db.lock().unwrap();

    match model::find_all_users(&mut conn) {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::InternalServerError().body("get users failed"),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let mut conn = db.lock().unwrap();

    match model::find_all_users(&mut conn) {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::InternalServerError().body("get users failed"),
    }
}

// 別にいらなかったけど、デバッグのために作った
#[post("/users")]
pub async fn post_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO: request bodyからnameを取得できるようにしてください
    let nuser = model::NewUser {
        name: "test".to_string(),
    };

    let mut conn = db.lock().unwrap();

    let user = model::create_user(&mut conn, nuser);
    match user {
        Ok(user) => HttpResponse::Ok().body(
            serde_json::to_string(&user).unwrap()
        ),
        Err(_e) => HttpResponse::InternalServerError().body("post user failed"),
    }
}

#[get("/questions")]
pub async fn get_questions(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let mut conn = db.lock().unwrap();

    match model::find_all_users(&mut conn) {
        // TODO: ここでuser_listを返してるけどquestionsを返すようにしたい
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),
        Err(_e) => HttpResponse::InternalServerError().body("get questions failed"),
    }
}

// TODO questions/:id/answersのGETを追加する
// 用途は、この質問に対する回答一覧を取得します

#[post("/answers")]
pub async fn post_answers(_: String) -> impl Responder {
    // TODO: questionを登録する
    // user name, question_id, answerを受け取る
    // user nameからusersテーブルをfindして、なかったら作る
    // 同じuser nameでは登録できないようにしたいので、migrationでunique制約をつける
    // user_idを取得する
    // answersテーブルはuser_id, question_id, answer

    HttpResponse::Ok().body("post answers")
}

