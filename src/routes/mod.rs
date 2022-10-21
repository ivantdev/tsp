pub mod singup;
pub mod login;

use crate::{algo::shortest_paths, global::Data, utils::Coordinate};
use rocket::{
    post,
    response::status,
    serde::{json::Json, Deserialize},
    State,
};
use serde::Serialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DijkstraInput<'r> {
    pub source: &'r str, // source node given as a
    pub destination: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DijkstraOutput {
    pub path: Vec<String>,
    pub distance: u32,
}

#[post("/shortestpath", data = "<input>")]
pub fn shortestpath( input: Json<DijkstraInput<'_>>, state: &State<Data>, ) -> Result<Json<DijkstraOutput>, status::BadRequest<String>> {
    let source = input.source;
    let destination = input.destination;

    //try to approximate coordinate using kd-tree
    let src_inf: Vec<&str> = source.split(' ').collect();
    let dest_inf: Vec<&str> = destination.split(' ').collect();
    if src_inf.len() != 2 || dest_inf.len() != 2 {
        return Err(status::BadRequest(Some(
            "Invalid source or destination".to_string(),
        )));
    }
    let src_lat: f64 = src_inf[0].parse().unwrap();
    let src_long: f64 = src_inf[1].parse().unwrap();
    let dest_lat: f64 = dest_inf[0].parse().unwrap();
    let dest_long: f64 = dest_inf[1].parse().unwrap();
    let src_item = state.kd_tree.nearest(&[src_lat, src_long]).unwrap();
    let dest_item = state.kd_tree.nearest(&[dest_lat, dest_long]).unwrap();
    let source = src_item.item;
    let destination = dest_item.item;
    let source = Coordinate {
        latitude: source[0],
        longitude: source[1],
    };
    let destination = Coordinate {
        latitude: destination[0],
        longitude: destination[1],
    };

    let source = state.map_coordinates_to_id.get(&source.to_string());
    let destination = state.map_coordinates_to_id.get(&destination.to_string());

    match (source, destination) {
        (Some(source), Some(destination)) => {
            let source = *source;
            let destination = *destination;
            let shortest_path =
                shortest_paths::dijkstra(&state.graph, source, destination).unwrap();
            let path = shortest_paths::reconstruct_path(shortest_path.1, destination).unwrap();
            let path = path
                .iter()
                .map(|x| state.map_id_to_coordinates.get(x).unwrap().to_string())
                .collect();
            Ok(Json(DijkstraOutput {
                path,
                distance: shortest_path.0,
            }))
        }
        _ => {
            return Err(status::BadRequest(Some(
                "Invalid source or destination".to_string(),
            )))
        }
    }
}
