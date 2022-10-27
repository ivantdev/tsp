use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

impl Coordinate {
    pub fn to_string(&self) -> String {
        let latitude = self.lat.to_string();
        let longitude = self.lng.to_string();
        latitude + " " + &longitude
    }
}