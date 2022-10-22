use rocket::{post, http::Status, response::status::Custom};
use rocket::serde::{Deserialize, json::Json};
use crate::db::users::get_user;
use crate::utils::{hash::hash_password, response::*, claims::Claims};
use jsonwebtoken::{encode, Header, EncodingKey, get_current_timestamp};
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Body<'r> {
    pub email: &'r str,
    pub password: &'r str
}

#[post("/", data="<body>")]
pub fn login(body: Json<Body<'_>>) -> Result<Json<OkResponse>, Custom<Json<ErrorResponse>>> {
    let query_response = get_user(body.email);
    let token: String;
    
    match query_response {
        Ok(user) => {
            let user = &user[0];
            let password_hashed: String = hash_password(&user.salt, body.password);

            if password_hashed == user.password {

                let claims: Claims = Claims { 
                    uid: user.id,
                    username: user.username.to_string(),
                    iat: get_current_timestamp(),
                    exp: get_current_timestamp() + 1814400
                };

                dotenv().ok();
                let secret: String = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
                token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

                let response: OkResponse = OkResponse {
                    message: "authentication successful".to_string(),
                    token: Some(token),
                    username: Some(user.username.clone())
                };
                Ok(Json(response))
            } else {
                let response: ErrorResponse = ErrorResponse {
                    message: "authentication failed".to_string()
                };
                Err(Custom(Status::Unauthorized, Json(response)))
            }
        },
        Err(error) => {
            let response: ErrorResponse = ErrorResponse {
                message: format!("error: {:?}", error)
            };
            Err(Custom(Status::Unauthorized, Json(response)))
        },
    }

}