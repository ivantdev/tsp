use serde::{Serialize, Deserialize};

use crate::utils::Coordinate;

#[derive(Serialize, Deserialize, Debug)]
pub struct  PathLocation {
    pub location: Coordinate,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub title: String,
    pub path: Vec<Coordinate>,
    pub distance: f64,
    pub locations: Vec<PathLocation>
}