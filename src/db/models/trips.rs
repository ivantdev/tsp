use crate::schema::trips;
use crate::db::models::users::User;
use chrono::{self, NaiveDateTime};
use diesel::{prelude::*, dsl};
use serde_json;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = trips)]
pub struct Trip {
    pub id: i32,
    pub user_id: i32,
    pub title: Option<String>,
    pub locations: serde_json::Value,
    pub path: serde_json::Value,
    pub distance: f64,
    pub completed: bool,
    pub created_on: NaiveDateTime
}

#[derive(Insertable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = trips)]
pub struct NewTrip<'a> {
    pub user_id: &'a i32,
    pub title: &'a str,
    pub locations: &'a serde_json::Value,
    pub path: &'a serde_json::Value,
    pub distance: &'a f64,
    pub completed: &'a bool,
    pub created_on: &'a dsl::now,
}