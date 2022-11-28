use crate::{
    algo::{tsp_solver::TspSolver, shortest_paths::{harvesine_heuristic, astar, reconstruct_path}},
    global::Data,
    utils::{
        auth_token::Token, authenticate::{authenticate, get_claims_by_token}, coordinate::Coordinate,
        response::ErrorResponse, trip::{Trip, Location}, path::{Path, PathLocation},
    }, db::{trips::create_trip, users::get_user_by_id},
};
use rocket::{http::Status, post, response::status::Custom, serde::json::Json, State};
use serde::Serialize;
use std::error::Error;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DijkstraOutput {
    pub path: Vec<Coordinate>,
    pub distance: f64,
}

#[post("/shortestpath", data = "<data>")]
pub fn shortestpath(
    token_raw: Token,
    mut data: Json<Trip>,
    state: &State<Data>,
) -> Result<Json<Path>, Custom<Json<ErrorResponse>>> {
    let token_raw = token_raw.tkn.split(' ').collect::<Vec<&str>>()[1];
    if authenticate(token_raw) {
        let mut nodes: Vec<Coordinate> = Vec::new();
        for i in 0..data.locations.len() {
            data.locations[i].coordinates.id = data.locations[i].id;
            nodes.push(data.locations[i].coordinates);
        }
        
        let mut tsp = TspSolver::new(&state.graph, &state.map_id_to_coordinates, nodes);
        let results = tsp.held_karp_solve().unwrap();

        let mut new_locations: Vec<Location> = Vec::new();
        for i in 0..results.len() {
            for e in 0..data.locations.len() {
                if results[i] == data.locations[e].id {
                    new_locations.push(data.locations[e].clone());
                }
            }
        }
        data.locations = new_locations;

        let distance_path = build_path(&data.locations, &state);
        let response = match distance_path {
            Ok(d_p) => {
                let mut path_aux: Vec<PathLocation> = Vec::new();
                for i in 0..data.locations.len() {
                    path_aux.push(PathLocation { location: data.locations[i].coordinates, label: format!("{}", i+1) });
                }
                Path {
                    title: data.title.clone(),
                    path: d_p.1,
                    distance: d_p.0,
                    locations: path_aux
                }
            },
            Err(message) => {
                let response: ErrorResponse = ErrorResponse {
                    message: message.to_string(),
                };
                return Err(Custom(Status::BadRequest, Json(response)));
            }
        };
        let token_claims = get_claims_by_token(token_raw).unwrap();
        let user = get_user_by_id(&token_claims.uid).unwrap();

        let _created_trip = create_trip(
            &user.id,
            &data.title,
            &data.locations,
            &response,
            &response.distance,
            &false,
            &diesel::dsl::now
        ).unwrap();

        Ok(Json(response))

    } else {
        let response: ErrorResponse = ErrorResponse {
            message: "Invalid session token".to_string(),
        };
        Err(Custom(Status::Unauthorized, Json(response)))
    }
}

pub fn approximate_coordinate(state: &State<Data>, coordinate: &Coordinate) -> Vec<Vec<f64>> {
    //try to approximate coordinate using kd-tree
    let latitude: f64 = coordinate.lat;
    let longitude: f64 = coordinate.lng;
    let nodes = state
        .kd_tree
        .get_n_nearest_neighbor(&vec![latitude, longitude], 5)
        .unwrap();
   nodes
}

pub fn build_path(path: &Vec<Location>, state: &State<Data>) -> Result<(f64, Vec<Coordinate>), Box<dyn Error>> {
    let mut new_path: Vec<Coordinate> = vec![];
    let mut distance:f64 = 0.;
    for i in 0..path.len() - 1 {

        let start_approximation = approximate_coordinate(state, &path[i].coordinates);
        let end_approximation: Vec<Vec<f64>> = approximate_coordinate(state, &path[i+1].coordinates);

        let mut resolved_path: bool = false;
        for e in 0..start_approximation.len() {
            if resolved_path { break; }

            let min_len = if e < end_approximation.len() { e } else { end_approximation.len() };
            for j in 0..min_len {
                let src: usize = start_approximation[e][2] as usize;
                let dest: usize = end_approximation[j][2] as usize;
                let dijkstra_result = astar(&state.graph, &state.map_id_to_coordinates, src, dest, &harvesine_heuristic);
                
                match dijkstra_result {
                    Ok(ok_path) => {
                        distance += ok_path.0;
                        let results = reconstruct_path(ok_path.1, dest).unwrap();

                        for r in results {
                            let node = state.map_id_to_coordinates.get(&r).unwrap().clone();
                            new_path.push(node);
                        }
                        resolved_path = true;
                        break;
                    },
                    Err(_) => {
                        continue;
                    }
                }
            }
        }

        if !resolved_path {
            return Err("No path found".into())
        }
    }

    return Ok((distance, new_path));
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
