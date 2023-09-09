use std::sync::Mutex;

use actix_web::{get, HttpResponse, post, Responder, web};
use diesel::pg::PgConnection;

#[get("/")]
pub async fn get_root(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO /get_usersと同じことをしたいので、共通化してください
    // TODO crate::model::って書き方が何か違う気がする
    match crate::model::find_all_users(db).await {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("root failed"),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match crate::model::find_all_users(db).await {
        Ok(user_list) => HttpResponse::Ok().body(
            serde_json::to_string(&user_list).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("failed"),
    }
}

// 別にいらなかったけど、デバッグのために作った
#[post("/users")]
pub async fn post_users(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    // TODO: request bodyからnameを取得できるようにしてください
    let nuser = crate::model::NewUser {
        name: "test".to_string(),
    };

    let user = crate::model::create_user(db, nuser).await;
    match user {
        Ok(user) => HttpResponse::Ok().body(
            serde_json::to_string(&user).unwrap()
        ),

        // TODO: HttpResponse::Errorを返せるようにしてください
        Err(_e) => HttpResponse::Ok().body("post user failed"),
    }
}

#[get("/questions")]
pub async fn get_questions(db: web::Data<Mutex<PgConnection>>) -> impl Responder {
    match crate::model::find_all_users(db).await {
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
pub async fn post_answers(_: String) -> impl Responder {
    // TODO: questionを登録する
    // user_name, question_id, answerを受け取る
    // user_nameからusersテーブルをfindして、なかったら作る
    // user_idを取得する
    // answersテーブルはuser_id, question_id, answer

    HttpResponse::Ok().body("post answers")
}
