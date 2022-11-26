use rocket::{get, response::status::Custom, serde::json::Json, http::Status};
use crate::{utils::{auth_token::Token, user::UserBriefDetails, response::ErrorResponse, authenticate::{authenticate, get_claims_by_token}}, db::users::get_user_by_id};

#[get("/")]
pub fn get_user_details(token_raw: Token) -> Result<Json<UserBriefDetails>, Custom<Json<ErrorResponse>>> {
    let token_raw = token_raw.tkn.split(' ').collect::<Vec<&str>>()[1];
    if authenticate(token_raw) {
        let token_claims = get_claims_by_token(token_raw).unwrap();
        let user = get_user_by_id(&token_claims.uid).unwrap();

        if user.username == token_claims.username {
            let user_details = UserBriefDetails {
                name: user.name,
                username: user.username,
                email: user.email,
                picture: user.picture
            };

            return Ok(Json(user_details))
        }
    }
    let response = ErrorResponse {
        message: "Invalid Token".to_string(),
    };
    Err(Custom(Status::Unauthorized, Json(response)))
}