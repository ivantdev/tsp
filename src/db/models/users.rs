use crate::schema::users;
use chrono::{self, NaiveDateTime};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub salt: String,
    pub password: String,
    pub picture: Option<String>,
    pub created_on: NaiveDateTime,
    pub staff: bool,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub salt: &'a str,
    pub password: &'a str,
    pub created_on: &'a diesel::dsl::now,
}
