use serde::{Serialize, Deserialize};

use crate::utils::Coordinate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Locations {
    pub locations: Vec<Coordinate>,
}