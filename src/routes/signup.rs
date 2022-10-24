use diesel;
use regex::Regex;
use rocket::{post, http::Status, response::status::Custom};
use rocket::serde::{Deserialize, json::Json};
use crate::db::users::create_user;
use crate::utils::{salt::gen_salt, hash::hash_password, response::*, claims::Claims};
use jsonwebtoken::{encode, Header, EncodingKey, get_current_timestamp};
use dotenvy::dotenv;
use std::env;

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
    email_regex.is_match(email)
}

#[post("/", data="<body>")]
pub fn sign_up(body: Json<Body<'_>>) -> Result<Json<OkResponse>, Custom<Json<ErrorResponse>>>{
    if !check_email(body.email) {
        let response = ErrorResponse {
            message: "Wrong data. Please check your data".to_string()
        };
        return Err(Custom(Status::BadRequest, Json(response)))
    }
    
    let salt: String = gen_salt();
    let password_hashed: String = hash_password(&salt, body.password);
    match create_user(
            &body.name.to_string(),
            &body.username.to_string(),
            &body.email.to_string(),
            &salt,
            &password_hashed,
            &diesel::dsl::now
        ) {
            Ok(user) => {
                let user = &user[0];
                let claims: Claims = Claims { 
                    uid: user.id,
                    username: user.username.to_string(),
                    iat: get_current_timestamp(),
                    exp: get_current_timestamp() + 1814400
                };

                dotenv().ok();
                let secret: String = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();            


                let response = OkResponse{
                    message: "successful register".to_string(),
                    token: Some(token),
                    username: Some(user.username.clone())
                };
                Ok(Json(response))
            },
            Err(error) => {
                let message: String;
                println!("obtuvimos el error: {:?}", error);
                let error_raw = format!("{:?}", error);
                if error_raw.contains("UniqueViolation") {
                    if error_raw.contains("username") {
                        message = "username already used".to_string();
                    } else if error_raw.contains("email") {
                        message = "email already used".to_string();
                    } else {
                        message = "Unexpected error".to_string();
                    }

                } else {
                    message = "Unexpected error".to_string();
                }
                println!("obtuvimos el error: {:?}", error_raw.contains("username"));

    
                let response = ErrorResponse {
                    message
                };
                Err(Custom(Status::BadRequest, Json(response)))
            },
        
    }
}
