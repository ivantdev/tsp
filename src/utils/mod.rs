pub mod auth_token;
pub mod authenticate;
pub mod claims;
pub mod coordinate;
pub mod hash;
pub mod response;
pub mod salt;
pub mod trip;
pub mod path;
pub mod user;

pub use crate::ds::{graph::Graph, kdtree::KdTree};
use coordinate::Coordinate;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn create_adjacency_list_from_files(
    coordinates_file: &String,
    arcs_file: &String,
) -> Result<Graph, Box<dyn Error>> {
    let coordinates_file = fs::read_to_string(coordinates_file)?;
    let arcs_file = fs::read_to_string(arcs_file)?;

    let mut num_of_nodes = 0;
    for _line in coordinates_file.lines() {
        num_of_nodes += 1;
    }

    let mut edges = vec![vec![]; num_of_nodes];
    let mut weights = vec![vec![]; num_of_nodes];

    for line in arcs_file.lines() {
        let mut split_line = line.split_whitespace();
        let source: usize = split_line.next().unwrap().parse()?;
        let destination: usize = split_line.next().unwrap().parse()?;
        let weight = split_line.next().unwrap().parse::<f64>()?;
        edges[source].push(destination);
        weights[source].push(weight);
    }

    Ok(Graph { edges, weights })
}

pub fn create_id_to_coordinates_hashmap_from_file(
    coordinates_file: &str,
) -> Result<HashMap<usize, Coordinate>, Box<dyn Error>> {
    let file = fs::read_to_string(coordinates_file)?;
    let mut coordinates_hashmap = HashMap::new();
    for line in file.lines() {
        let mut split_line = line.split_whitespace();
        let id = split_line.next().unwrap().parse::<usize>()?;
        let latitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();
        let longitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();

        let coordinate = Coordinate {
            lat: latitude,
            lng: longitude,
            id: id,
        };

        coordinates_hashmap.insert(id, coordinate);
    }

    Ok(coordinates_hashmap)
}

pub fn create_kd_tree_from_file(
    coordinates_file: &str,
) -> Result<KdTree<f64>, Box<dyn Error>> {
    let file = fs::read_to_string(coordinates_file)?;
    let mut points = vec![];
    for line in file.lines() {
        let mut split_line = line.split_whitespace();
        let id = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();
        let latitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();
        let longitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();

        points.push(vec![latitude, longitude, id])
    }
    let tree = KdTree::new(points);
    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_adjacency_list_from_files_correct() {
        let _graph = create_adjacency_list_from_files(
            &("nodes.txt".to_string()),
            &("edges.txt".to_string()),
        )
        .unwrap();
    }

    #[test]
    fn create_kd_tree_from_file_correct() {
        let _tree = create_kd_tree_from_file(&("nodes.txt".to_string())).unwrap();
    }
}
