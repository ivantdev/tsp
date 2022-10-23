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
pub fn shortestpath(
    input: Json<DijkstraInput<'_>>,
    state: &State<Data>,
) -> Result<Json<DijkstraOutput>, status::BadRequest<String>> {
    let source = input.source;
    let destination = input.destination;

    let source = approximate_coordinate(state, source)?;
    let destination = approximate_coordinate(state, destination)?;

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
        _ => Err(status::BadRequest(Some(
            "Invalid source or destination".to_string(),
        ))),
    }
}

fn approximate_coordinate(
    state: &State<Data>,
    coordinate: &str,
) -> Result<Coordinate, status::BadRequest<String>> {
    //try to approximate coordinate using kd-tree
    let coordinate: Vec<&str> = coordinate.split(' ').collect();
    if coordinate.len() != 2 {
        return Err(status::BadRequest(Some(
            "Invalid source or destination".to_string(),
        )));
    }
    let latitude: f64 = coordinate[0].parse().unwrap();
    let longitude: f64 = coordinate[1].parse().unwrap();
    let coordinate_and_distance = state.kd_tree.nearest(&[latitude, longitude]).unwrap();
    let coordinate = coordinate_and_distance.item;
    let coordinate = Coordinate {
        latitude: coordinate[0],
        longitude: coordinate[1],
    };
    Ok(coordinate)
}
