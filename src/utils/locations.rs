use serde::{Serialize, Deserialize};

use crate::utils::trip::Location;

#[derive(Serialize, Deserialize, Debug)]
pub struct Locations {
    pub locations: Vec<Location>,
}