use crate::algo::shortest_paths::dijkstra;
use crate::ds::graph::Graph;
use geoutils::Location;
pub struct TspSolver {
    pub road_network: Graph,
    pub path: Vec<usize>,
    pub distance: f64,
    pub nodes: Vec<usize>,
}

impl TspSolver {
    pub fn new(road_network: Graph, nodes: Vec<usize>) -> Self {
        Self {
            nodes,
            path: vec![],
            distance: 0.0,
            road_network,
        }
    }

    // it is assume that the first node is the starting node
    pub fn solve(&mut self) {
        let complete_graph = self.reduce_to_complete_graph();
        // let mut path = vec![];
        // let mut distance = 0.0;
        // let mut current_node = 0;
    }

    // compute the distance between every pair of nodes and store it in the graph
    pub fn reduce_to_complete_graph(&mut self) -> Graph {
        let mut graph = Graph::new(self.nodes.len());
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if i != j {
                    let distance = dijkstra(&self.road_network, self.nodes[i], self.nodes[j])
                        .unwrap()
                        .0;
                    graph.add_edge(self.nodes[i], self.nodes[j], distance);
                }
            }
        }
        graph
    }
}
