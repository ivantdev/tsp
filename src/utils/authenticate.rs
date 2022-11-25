use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use dotenvy::dotenv;
use std::env;

use crate::db::users::get_user;

use super::claims::Claims;

pub fn authenticate(token: &str) -> bool {
    dotenv().ok();
    let secret = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));
    match token_data {
        Ok(token) => {
            let claims = token.claims;
            if claims.exp < claims.iat {
                return false;
            }
            get_user(&claims.username).is_ok()
        },
        Err(_) => {
            false
        }
    }
}

pub fn get_id_user_by_token(token: &str) -> Result<Claims, &str> {
    dotenv().ok();
    let secret = env::var("SECRET_JWT").expect("SECRET_JWT must be set");
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

    match token_data {
        Ok(token) => {
            Ok(token.claims)
        },
        Err(_) => Err("Invalid Token")
    }

}