use serde::Serialize;



#[derive(Serialize, Debug)]
pub struct UserBriefDetails {
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub picture: Option<String>,
}