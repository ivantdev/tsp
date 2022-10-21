use rocket::{post, http::Status, response::status::Custom};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, Serialize, json::Json};
use crate::db::users::get_user;
use crate::utils::{hash::hash_password, response::*};
use crate::db::models::users::User;
use jsonwebtoken::{encode, Header, EncodingKey};
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Body<'r> {
    pub email: &'r str,
    pub password: &'r str
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    uid: i32,
    username: String,
}

#[post("/", data="<body>")]
pub fn login(body: Json<Body<'_>>, jar: &CookieJar) -> Result<Json<OkResponse>, Custom<Json<ErrorResponse>>> {
    let query_response = get_user(body.email).unwrap();
    let mut status: bool = false;
    let mut token: String;
    
    if query_response.len() > 0 {

        let user: &User = &query_response[0];
        let password_hashed: String = hash_password(&user.salt, body.password);

        if password_hashed == user.password {

            status = true;

            let claims: Claims = Claims { uid: user.id.to_owned(), username: user.username.to_string().to_owned() };
            dotenv().ok();

            let secret: String = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
            token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
            token = format!("Bearer {token}");

            let cookie = Cookie::new("token", token);
            jar.add(cookie);
            
        }

    }

    if status {
        let response: OkResponse = OkResponse {
            message: format!("authentication successful")
        };
        return Ok(Json(response));
    } else {
        let response: ErrorResponse = ErrorResponse {
            message: format!("failed authentication")
        };
        return Err(Custom(Status::Unauthorized, Json(response)));
    }

}