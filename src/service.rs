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
    // でもどっからrequest bodyが取り出せるんだ・・・？
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

// TODO: questions/:id/answersのGETを追加する
// 用途は、この質問に対する回答一覧を取得します


#[post("/questions/{questions_id}/answers")]
pub async fn post_answers(db: web::Data<Mutex<PgConnection>>, web::Path(questions_id): web::Path<i32>, web::Json(answer): web::Json<model::NewAnswer>) -> impl Responder {
    let mut conn = db.lock().unwrap();

    let question = model::find_question(&mut conn, question_id);
    match question {
        Ok(question) => {
            let user = model::find_or_create_user(&mut conn, answer.user_name.clone());
            match user {
                Ok(user) => {
                    let nanswer = model::NewAnswer {
                        users_id: user.id,
                        questions_id: question.id,
                        answer: answer.answer,
                    };
                    let answer = model::answer_question(&mut conn, nanswer);
                    match answer {
                        Ok(answer) => HttpResponse::Ok().body(
                            serde_json::to_string(&answer).unwrap()
                        ),
                        Err(_e) => HttpResponse::InternalServerError().body("post answer failed"),
                    }
                },
            }
        },
        Err(_e) => HttpResponse::InternalServerError().body("post answer failed"),
    }
}

// TODO: users/:id/answersのGETを追加する
// ユーザは自分の回答一覧をみたいはずなので。
// ただ、上記の仕様ではユーザは自分のuser_idを知らないはずなので、 users/:name/answersというルーティングにしても良いかもしれない
// その場合はユーザ名は英数字のみみたいな規約を追加することに
