use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct OkResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String
}