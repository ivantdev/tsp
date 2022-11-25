use crate::ds::graph::Graph;
use crate::utils::coordinate::Coordinate;
use geoutils::Location;
use std::collections::HashMap;
pub struct TspSolver<'a> {
    pub road_network: &'a Graph,
    pub id_to_coordinates: &'a HashMap<usize, Coordinate>,
    pub path: Vec<usize>,
    pub distance: f64,
    pub nodes: Vec<usize>,
}

impl<'a> TspSolver<'a> {
    pub fn new(
        road_network: &'a Graph,
        id_to_coordinates: &'a HashMap<usize, Coordinate>,
        nodes: Vec<usize>,
    ) -> Self {
        Self {
            nodes,
            path: vec![],
            distance: 0.0,
            road_network,
            id_to_coordinates,
        }
    }

    // it is assume that the first node is the starting node
    pub fn solve(&mut self) {
        let complete_graph = self.reduce_to_complete_graph();
    }

    // compute the distance between every pair of nodes and store it in the graph
    pub fn reduce_to_complete_graph(&mut self) -> Graph {
        let mut graph = Graph::new(self.nodes.len());
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if i != j {
                    let i_coord = self.id_to_coordinates.get(&i).unwrap();
                    let j_coord = self.id_to_coordinates.get(&j).unwrap();
                    let i_location = Location::new(i_coord.lat, i_coord.lng);
                    let j_location = Location::new(j_coord.lat, j_coord.lng);
                    let distance = i_location.haversine_distance_to(&j_location);
                    graph.add_edge(self.nodes[i], self.nodes[j], distance.meters());
                }
            }
        }
        graph
    }
}
