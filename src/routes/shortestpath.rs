use crate::{
    algo::{shortest_paths, tsp_solver::TspSolver},
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
        let mut id_nodes: Vec<usize> = Vec::new();
        for i in data.locations.iter() {
            let coordinate = &i.coordinates;
            let coordinate = approximate_coordinate(state, coordinate);
            id_nodes.push(coordinate.id);
        }

        let mut tsp = TspSolver::new(&state.graph, &state.map_id_to_coordinates, id_nodes);
        let results = tsp.held_karp_solve().unwrap();

        let mut path: Vec<Coordinate> = Vec::new();
        for i in results {
            let node = state.map_id_to_coordinates.get(&i).unwrap().clone();
            path.push(node);
        }
        Ok(Json(DijkstraOutput {
            path,
            distance: 0.0,
        }))
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
    let coordinate = state
        .kd_tree
        .nearest_neighbor(&state.kd_tree.root, &vec![latitude, longitude], 0)
        .unwrap();
    Coordinate {
        lat: coordinate[0],
        lng: coordinate[1],
        id: coordinate[2] as usize,
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::utils::compare_coordinates_approximation;

    #[test]
    #[ignore]
    fn test_compare_coordinates_approximation() {
        compare_coordinates_approximation();
    }
}
