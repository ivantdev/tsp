use crate::{
    algo::shortest_paths,
    global::Data,
    utils::{
        auth_token::Token, authenticate::authenticate, coordinate::Coordinate,
        response::ErrorResponse, trip::Trip,
    },
};
use rocket::{http::Status, post, response::status::Custom, serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DijkstraOutput {
    pub path: Vec<Coordinate>,
    pub distance: f64,
}

#[post("/shortestpath", data = "<data>")]
pub fn shortestpath(
    token_raw: Token,
    data: Json<Trip>,
    state: &State<Data>,
) -> Result<Json<DijkstraOutput>, Custom<Json<ErrorResponse>>> {
    let token_raw = token_raw.tkn.split(' ').collect::<Vec<&str>>()[1];
    if authenticate(token_raw) {
        let source = &data.locations[0].coordinates;
        let destination = &data.locations[1].coordinates;

        let source = approximate_coordinate(state, source);
        let destination = approximate_coordinate(state, destination);

        println!("source: {:?}", source);
        println!("destination: {:?}", destination);

        let source = state.map_id_to_coordinates.get(&source.id);
        let destination = state.map_id_to_coordinates.get(&destination.id);

        match (source, destination) {
            (Some(source), Some(destination)) => {
                let source = source;
                let destination = destination;
                let shortest_path =
                    shortest_paths::dijkstra(&state.graph, source.id, destination.id).unwrap();
                let path = shortest_paths::reconstruct_path(shortest_path.1, destination.id).unwrap();
                let path = path
                    .iter()
                    .map(|x| {
                        let coordenate = state
                            .map_id_to_coordinates
                            .get(x)
                            .unwrap();
                            Coordinate { lat: coordenate.lat, lng: coordenate.lng, id: coordenate.id }
                    }).collect();


                Ok(Json(DijkstraOutput {
                    path,
                    distance: shortest_path.0,
                }))
            }
            _ => {
                let response: ErrorResponse = ErrorResponse {
                    message: "Invalid locations".to_string(),
                };
                Err(Custom(Status::Unauthorized, Json(response)))
            }
        }
    } else {
        let response: ErrorResponse = ErrorResponse {
            message: "Invalid session token".to_string(),
        };
        Err(Custom(Status::Unauthorized, Json(response)))
    }
}

fn approximate_coordinate(state: &State<Data>, coordinate: &Coordinate) -> Coordinate {
    //try to approximate coordinate using kd-tree
    let latitude: f64 = coordinate.lat;
    let longitude: f64 = coordinate.lng;
    let coordinate = state.kd_tree.nearest_neighbor(&state.kd_tree.root, &vec![latitude, longitude], 0).unwrap();
    Coordinate {
        lat: coordinate[0],
        lng: coordinate[1],
        id: coordinate[2] as usize,
    }
}
