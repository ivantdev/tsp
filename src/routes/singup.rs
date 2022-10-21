use diesel;
use regex::Regex;
use rocket::{post, http::Status, response::status::Custom};
use rocket::serde::{Deserialize, json::Json};
use crate::db::users::create_user;
use crate::utils::{salt::gen_salt, hash::hash_password, response::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Body<'r> {
    name: &'r str,
    username: &'r str,
    email: &'r str,
    password: &'r str,
}

fn check_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(&email)
}

#[post("/", data="<body>")]
pub fn sing_up(body: Json<Body<'_>>) -> Result<Json<OkResponse>, Custom<Json<ErrorResponse>>>{
    if check_email(&body.email) {
        let salt: String = gen_salt();
        let password_hashed: String = hash_password(&salt, body.password);
        let _new_user = create_user(
            &body.name.to_string(),
            &body.username.to_string(),
            &body.email.to_string(),
            &salt,
            &password_hashed,
            &diesel::dsl::now
        );
        let response = OkResponse{
            message: format!("successful"),
        };
        return Ok(Json(response));
    }
    let response = ErrorResponse {
        message: "wrong data".to_string()
    };
    Err(Custom(Status::BadRequest, Json(response)))
}