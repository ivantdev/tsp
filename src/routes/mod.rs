use crate::{algo::shortest_paths, global::Data};
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
    //try to find the source and destination in the hashmap
    let source = state.map_coordinates_to_id.get(source);
    let destination = state.map_coordinates_to_id.get(destination);
    println!("source: {:?}", source);
    println!("destination: {:?}", destination);
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
