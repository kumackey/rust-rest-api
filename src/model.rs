use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use chrono;

use crate::schema::users;
use crate::schema::users::dsl::*;

use crate::schema::answers;
use crate::schema::answers::dsl::*;

use crate::schema::questions;
use crate::schema::questions::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

pub fn find_all_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    let results = users
        .limit(100)
        .load::<User>(conn)?;

    Ok(results)
}

pub fn create_user(conn: &mut PgConnection, user: NewUser) -> Result<User, diesel::result::Error> {
    let result = diesel::insert_into(users)
        .values(&user)
        .get_result(&mut *conn)?;

    Ok(result)
}

pub fn find_or_create_user(conn: &mut PgConnection, user_name: String) -> Result<User, diesel::result::Error> {
    let result = users
        .filter(name.eq(user_name))
        .first::<User>(conn);

    match result {
        Ok(user) => Ok(user),
        Err(_) => {
            let new_user = NewUser {
                name: user_name,
            };
            create_user(conn, new_user)
        }
    }
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

#[derive(Queryable, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub questioner_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub question: String,
}

pub fn find_question(conn: &mut PgConnection, question_id: i32) -> Result<Question, diesel::result::Error> {
    questions.find(question_id).first::<Question>(conn)
}

#[derive(Queryable, Serialize, Deserialize)]
struct Answer {
    id: i32,
    users_id: i32,
    questions_id: i32,
    answer: String,
    answered_at: chrono::NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub users_id: i32,
    pub questions_id: i32,
    pub answer: String,
    pub answered_at: chrono::NaiveDateTime
}

pub fn answer_question(conn: &mut PgConnection, new_answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let result = diesel::insert_into(answers)
        .values(&new_answer)
        .get_result(&mut *conn)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_find_all_users() -> Result<(), Box<dyn std::error::Error>> {
        // TODO: なんかこのテスト動かない・・・あとは任せた・・・
        // 最悪、テストなのでなくても良いけど・・・
        // Done is better than perfect.
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn = PgConnection::establish(&database_url)?;

        // TODO: bulk insert的なのあるはず？
        let _: User = diesel::insert_into(users)
            .values(&NewUser { name: "User1".to_string() })
            .get_result(&mut conn)?;
        let _: User = diesel::insert_into(users)
            .values(&NewUser { name: "User2".to_string() })
            .get_result(&mut conn)?;

        let result = find_all_users(&mut conn);

        assert!(result.is_ok());
        let us = result.unwrap();

        // TODO: 一度userレコードを消さないと数が一致しないので、なんとかしてください
        assert_eq!(us.len(), 2);
        assert_eq!(us[0].id, 1);
        assert_eq!(us[0].name, "User1");
        assert_eq!(us[1].id, 2);
        assert_eq!(us[1].name, "User2");

        Ok(())
    }
}
