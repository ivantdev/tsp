#[macro_use]
extern crate rocket;
extern crate queues;
use tsp::{global::Data, routes::shortestpath, utils};

#[launch]
fn rocket() -> _ {
    let coordinates_file = "USA-road-d.NY.co";
    let arcs_file = "USA-road-d.NY.gr";
    let graph = utils::create_adjacency_list_from_files(coordinates_file, arcs_file).unwrap();
    let map_coordinates_to_id =
        utils::create_coordinates_hashmap_from_file(coordinates_file).unwrap();
    let map_id_to_coordinates =
        utils::create_id_to_coordinates_hashmap_from_file(coordinates_file).unwrap();
    let state = Data {
        graph,
        map_coordinates_to_id,
        map_id_to_coordinates,
    };
    rocket::build()
        .manage(state)
        .mount("/", routes![shortestpath])
}
