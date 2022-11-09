#[macro_use]
extern crate rocket;
extern crate queues;
use dotenvy::dotenv;
use rocket::fs::NamedFile;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;
use std::path::{Path, PathBuf};
use tsp::routes::{login::login, shortestpath::shortestpath, signup::sign_up};
use tsp::{global::Data, utils};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("templates/index.html").await.ok()
}

#[get("/<file..>", rank = 1)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let coordinates_file = env::var("COORDINATES_FILE").unwrap();
    let arcs_file = env::var("ARCS_FILE").unwrap();
    let graph = utils::create_adjacency_list_from_files(&coordinates_file, &arcs_file).unwrap();
    let map_coordinates_to_id =
        utils::create_coordinates_hashmap_from_file(&coordinates_file).unwrap();
    let map_id_to_coordinates =
        utils::create_id_to_coordinates_hashmap_from_file(&coordinates_file).unwrap();
    let kd_tree = utils::create_kd_tree_from_file(&coordinates_file).unwrap();

    let state = Data {
        graph,
        map_coordinates_to_id,
        map_id_to_coordinates,
        kd_tree,
    };

    let allowed_origins = AllowedOrigins::some_exact(&[env::var("FRONTEND_URL").unwrap()]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .manage(state)
        .mount("/", routes![shortestpath, index, files])
        .mount("/map", routes![index])
        .mount("/history", routes![index])
        .mount("/new-trip", routes![index])
        .mount("/signup", routes![index, sign_up])
        .mount("/login", routes![login, index])
        .attach(cors.to_cors().unwrap())
}
