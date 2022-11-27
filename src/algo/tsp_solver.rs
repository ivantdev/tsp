use crate::algo::shortest_paths::{astar, harvesine_heuristic, reconstruct_path};
use crate::ds::graph::Graph;
use crate::utils::coordinate::Coordinate;
use geoutils::Location;
use std::{collections::HashMap, error::Error};

pub struct TspSolver<'a> {
    pub road_network: &'a Graph,
    pub id_to_coordinates: &'a HashMap<usize, Coordinate>,
    pub path: Vec<usize>,
    pub distance: f64,
    pub nodes: Vec<Coordinate>,
    pub new_nodes_to_original_nodes: HashMap<usize, usize>,
}

impl<'a> TspSolver<'a> {
    pub fn new(
        road_network: &'a Graph,
        id_to_coordinates: &'a HashMap<usize, Coordinate>,
        nodes: Vec<Coordinate>,
    ) -> Self {
        Self {
            nodes,
            path: vec![],
            distance: 0.0,
            road_network,
            id_to_coordinates,
            new_nodes_to_original_nodes: HashMap::new(),
        }
    }

    // it is assume that the first node is the starting node
    pub fn held_karp_solve(&mut self) -> Result<Vec<usize>, Box<dyn Error>> {
        let dists = self.get_distance_matrix();
        let n = dists.len();

        // Maps each subset of the nodes to the cost to reach that subset, as well
        // as what node it passed before reaching this subset.
        // Node subsets are represented as set bits.
        let mut C = HashMap::<(i32, usize), (f64, usize)>::new();

        // Set transition cost from initial state
        for k in 1..n {
            C.insert((1 << k, k), (dists[0][k], 0));
        }

        // Iterate subsets of increasing size
        for subset_size in 2..n {
            // Iterate over subsets of this size
            for subset in (0..1 << n).filter(|s| i32::count_ones(*s) == subset_size as u32) {
                // Iterate over last nodes in the subset
                for k in 1..n {
                    // Skip if k is not in the subset
                    if (subset & (1 << k)) == 0 {
                        continue;
                    }

                    // Find the minimum previous cost
                    let mut min_prev = f64::INFINITY;
                    let mut argmin_prev = 0;

                    for m in 1..n {
                        if (subset & (1 << m)) == 0 || m == k {
                            continue;
                        }

                        let subset_without_k = subset ^ (1 << k);
                        let cost = C.get(&(subset_without_k, m)).unwrap().0 + dists[m][k];
                        if cost < min_prev {
                            min_prev = cost;
                            argmin_prev = m;
                        }
                    }

                    C.insert((subset, k), (min_prev, argmin_prev));
                }
            }
        }

        // We're interested in all bits but the least significant (the start state)
        let mut subset = (1 << n) - 1 - 1;

        // Find the minimum cost to get back to the start
        let mut min_cost = f64::INFINITY;
        let mut parent = 0;
        for k in 1..n {
            let cost = C.get(&(subset, k)).unwrap().0 + dists[k][0];
            if cost < min_cost {
                min_cost = cost;
                parent = k;
            }
        }

        self.path.push(0);

        //Backtrack to find the actual path
        for _ in 0..n - 1 {
            self.path.push(parent);
            let tmp = C.get(&(subset, parent)).unwrap().1;
            subset = subset ^ (1 << parent);
            parent = tmp;
        }

        // Add the initial node to the end
        self.path.push(0);
        self.path.reverse();

        // transform path to original nodes
        self.path = self
            .path
            .iter()
            .map(|node| self.new_nodes_to_original_nodes[node])
            .collect();

        // return the actual path
        Ok(self.path.to_owned())
    }

    // compute the distance between every pair of nodes and store it in the graph
    fn get_distance_matrix(&mut self) -> Vec<Vec<f64>> {
        let mut distance_matrix = vec![vec![0.0; self.nodes.len()]; self.nodes.len()];
        for i in 0..self.nodes.len() {
            self.new_nodes_to_original_nodes.insert(i, self.nodes[i].id);
            for j in 0..self.nodes.len() {
                if i == j {
                    distance_matrix[i][j] = 0.0;
                } else {
                    let i_coord = &self.nodes[i];
                    let j_coord = &self.nodes[j];
                    let i_location = Location::new(i_coord.lat, i_coord.lng);
                    let j_location = Location::new(j_coord.lat, j_coord.lng);
                    let distance = i_location.haversine_distance_to(&j_location);
                    distance_matrix[i][j] = distance.meters();
                }
            }
        }
        distance_matrix
    }

    fn expand_path(&mut self) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut new_path = vec![];
        for i in 0..self.path.len() - 1 {
            let start = self.path[i];
            let end = self.path[i + 1];
            let path = astar(
                self.road_network,
                self.id_to_coordinates,
                start,
                end,
                &harvesine_heuristic,
            )
            .unwrap();
            let path = reconstruct_path(path.1, end).unwrap();
            new_path.extend(path);
        }
        return Ok(new_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{
        create_adjacency_list_from_files, create_id_to_coordinates_hashmap_from_file,
    };
    use dotenvy::dotenv;
    use std::env;

    #[test]
    fn test_tsp_solver() {
        dotenv().ok();
        let coordinates_file = env::var("COORDINATES_FILE").unwrap();
        let arcs_file = env::var("ARCS_FILE").unwrap();
        let graph = create_adjacency_list_from_files(&coordinates_file, &arcs_file).unwrap();
        let id_to_coordinates =
            create_id_to_coordinates_hashmap_from_file(&coordinates_file).unwrap();

        // let mut tsp_solver = TspSolver::new(
        //     &graph,
        //     &id_to_coordinates,
        //     vec![0, 1, 2, 3, 323, 7, 4, 6, 8, 9, 534, 3242, 5646, 2132, 4355],
        // );
        // let path = tsp_solver.held_karp_solve().unwrap();
    }
}
