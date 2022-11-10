use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
    pub id: usize
}
