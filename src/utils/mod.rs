use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn to_string(&self) -> String {
        let latitude = self.latitude.to_string();
        let longitude = self.longitude.to_string();
        latitude + " " + &longitude
    }
}

#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<Vec<usize>>,
    pub weights: Vec<Vec<u16>>,
}

pub fn create_adjacency_list_from_files(
    coordinates_file: &str,
    arcs_file: &str,
) -> Result<Graph, Box<dyn Error>> {
    let coordinates_file = fs::read_to_string(coordinates_file)?;
    let arcs_file = fs::read_to_string(arcs_file)?;

    let mut num_of_nodes = 0;
    for line in coordinates_file.lines() {
        if line.starts_with('v') {
            num_of_nodes += 1;
        }
    }

    let mut edges = vec![vec![]; num_of_nodes];
    let mut weights = vec![vec![]; num_of_nodes];

    for line in arcs_file.lines() {
        if !line.starts_with('a') {
            continue;
        }
        let mut split_line = line.split_whitespace();
        split_line.next(); // a
        let source: usize = split_line.next().unwrap().parse()?;
        let destination: usize = split_line.next().unwrap().parse()?;
        let weight = split_line.next().unwrap().parse::<u16>()?;
        edges[source - 1].push(destination - 1);
        weights[source - 1].push(weight);
    }

    Ok(Graph { edges, weights })
}

pub fn create_coordinates_hashmap_from_file(
    coordinates_file: &str,
) -> Result<HashMap<String, u32>, Box<dyn Error>> {
    let file = fs::read_to_string(coordinates_file)?;
    let mut coordinates_hashmap = HashMap::new();
    for line in file.lines() {
        if !line.starts_with('v') {
            continue;
        }
        let mut split_line = line.split_whitespace();
        split_line.next(); // v
        let id = split_line.next().unwrap().parse::<u32>()?;
        //
        let mut longitude = split_line.next().unwrap().to_string();
        let longitude = normalize_coordinate(&mut longitude, 2)?;
        let mut latitude = split_line.next().unwrap().to_string();
        let latitude = normalize_coordinate(&mut latitude, 2)?;

        let coordinate = Coordinate {
            latitude,
            longitude,
        };

        coordinates_hashmap.insert(coordinate.to_string(), id);
    }

    Ok(coordinates_hashmap)
}

// converts a coordinate string like "72346276" into a decimal number like 72.346276
// depending on the position argument to add the point
fn normalize_coordinate(
    coordinate: &mut String,
    position: usize,
) -> Result<f64, std::num::ParseFloatError> {
    if coordinate.starts_with("-") {
        coordinate.insert(position + 1, '.');
    } else {
        coordinate.insert(position, '.');
    }

    coordinate.as_str().parse::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_coordinate_correct() {
        let res = normalize_coordinate(&mut "17682718".to_string(), 2).unwrap();
        assert_eq!(res, 17.682718)
    }

    #[test]
    fn normalize_negative_coordinate_correct() {
        let res = normalize_coordinate(&mut "-17682718".to_string(), 2).unwrap();
        assert_eq!(res, -17.682718)
    }

    #[test]
    fn create_coordinates_hashmap_from_file_correct() {
        let map = create_coordinates_hashmap_from_file("USA-road-d.NY.co").unwrap();
        assert_eq!(map.contains_key("40.897199 -73.975982"), true)
    }

    #[test]
    fn create_adjacency_list_from_files_correct() {
        let graph =
            create_adjacency_list_from_files("USA-road-d.NY.co", "USA-road-d.NY.gr").unwrap();
        assert!(graph.edges[0].contains(&1));
        assert!(graph.edges[1].contains(&0));
        let pos = graph.edges[0].iter().position(|&dest| dest == 1).unwrap();

        assert_eq!(graph.weights[0][pos], 803);
    }
}