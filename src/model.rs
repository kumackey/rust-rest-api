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
