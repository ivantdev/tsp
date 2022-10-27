use rocket::serde::{Deserialize, Serialize};
use crate::utils::coordinate::Coordinate;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    pub address: String,
    pub address_complement: String,
    pub coordinates: Coordinate,
    pub place_id: String
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Trip {
    pub back_to_start: bool,
    pub title: String,
    pub locations: Vec<Location>,
}