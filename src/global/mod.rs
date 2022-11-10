use crate::{ds::{graph::Graph, kdtree::KdTree}, utils::coordinate::Coordinate};
use std::collections::HashMap;

pub struct Data {
    pub graph: Graph,
    pub map_id_to_coordinates: HashMap<usize, Coordinate>,
    pub kd_tree: KdTree<f64>,
}
