use rocket::http::Status;
use rocket::{get, serde::json::Json, response::status::Custom};
use crate::db::models::trips::Trip;
use crate::{utils::{response::{DataResponse, ErrorResponse}, auth_token::Token, authenticate::{authenticate, get_id_user_by_token}}, db::{users::get_user_by_id, trips::get_trips_by_user_id}};


#[get("/<page>")]
pub fn get_history(page: i64, token_raw: Token) -> Result< Json<DataResponse<Vec<Trip>>>, Custom<Json<ErrorResponse>>> {
    let token_raw = token_raw.tkn.split(' ').collect::<Vec<&str>>()[1];
    if authenticate(token_raw) {
        let token_claims = get_id_user_by_token(token_raw).unwrap();
        let user = get_user_by_id(&token_claims.uid).unwrap();

        if user.username == token_claims.username {
            let trips = get_trips_by_user_id(&user.id, page).unwrap();

            let response = DataResponse {
                data: trips,
            };
            return Ok(Json(response))
        }
    }
    let response = ErrorResponse {
        message: "Invalid Token".to_string(),
    };
    Err(Custom(Status::Unauthorized, Json(response)))
}