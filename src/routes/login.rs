use crate::db::models::users::User;
use crate::db::users::get_user;
use crate::utils::{hash::hash_password, response::*};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http::Status, post, response::status::Custom};
use std::env;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Body<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    uid: i32,
    username: String,
}

#[post("/", data = "<body>")]
pub fn login(
    body: Json<Body<'_>>,
    jar: &CookieJar,
) -> Result<Json<OkResponse>, Custom<Json<ErrorResponse>>> {
    let query_response = get_user(body.email).unwrap();
    let mut status: bool = false;
    let mut token: String;

    if !query_response.is_empty() {
        let user: &User = &query_response[0];
        let password_hashed: String = hash_password(&user.salt, body.password);

        if password_hashed == user.password {
            status = true;

            let claims: Claims = Claims {
                uid: user.id.to_owned(),
                username: user.username.to_string(),
            };
            dotenv().ok();

            let secret: String = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
            token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .unwrap();
            token = format!("Bearer {token}");

            let cookie = Cookie::new("token", token);
            jar.add(cookie);
        }
    }

    if status {
        let response: OkResponse = OkResponse {
            message: "authentication successful".to_string(),
        };
        Ok(Json(response))
    } else {
        let response: ErrorResponse = ErrorResponse {
            message: "failed authentication".to_string(),
        };
        Err(Custom(Status::Unauthorized, Json(response)))
    }
}
