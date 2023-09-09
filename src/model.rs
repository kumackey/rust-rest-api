use std::sync::Mutex;

use actix_web::web;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

pub async fn find_all_users(db: web::Data<Mutex<PgConnection>>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let results = users
        .limit(100)
        .load::<User>(&mut *conn)?;

    Ok(results)
}

pub async fn create_user(db: web::Data<Mutex<PgConnection>>, user: NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = db.lock().unwrap();

    let result = diesel::insert_into(users)
        .values(&user)
        .get_result(&mut *conn)?;

    Ok(result)
}

// TODO: 上に似たような感じで、以下の関数やモデルを作ってみて下さい。
// async fn find_by_name(db: web::Data<Mutex<PgConnection>>,name: String) -> Result<User, diesel::result::Error> {
//     let mut conn = db.lock().unwrap();
//
//     let result = users
//         .filter(name.eq(name))
//         .first::<User>(&mut *conn)?;
//
//     Ok(result)
// }

// async fn answer_question(db: web::Data<Mutex<PgConnection>>, answer: Answer) -> Result<Answer, diesel::result::Error> {
//     let mut conn = db.lock().unwrap();
//
//     let result = diesel::insert_into(answers)
//         .values(&answer)
//         .get_result(&mut *conn)?;
//
//     Ok(result)
// }

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

#[cfg(test)]
mod tests {
    async fn test_find_all_users() {

    }
}
