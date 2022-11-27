use rocket::serde::{Deserialize, Serialize};
use crate::utils::coordinate::Coordinate;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    pub address: String,
    pub address_complement: String,
    pub coordinates: Coordinate,
    pub place_id: String,
    pub id: usize,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Trip {
    pub back_to_start: bool,
    pub title: String,
    pub locations: Vec<Location>,
}