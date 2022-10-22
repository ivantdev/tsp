use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uid: i32,
    pub username: String,
    pub exp: u64,
    pub iat: u64,
}