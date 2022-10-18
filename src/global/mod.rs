use crate::ds::graph::Graph;
use std::collections::HashMap;

pub struct Data {
    pub graph: Graph,
    pub map_coordinates_to_id: HashMap<String, usize>,
    pub map_id_to_coordinates: HashMap<usize, String>,
}
