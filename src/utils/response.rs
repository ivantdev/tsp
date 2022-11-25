use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct OkResponse {
    pub message: String,
    pub token: Option<String>,
    pub username: Option<String>
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct DataResponse<T> {
    pub data: T,
}
