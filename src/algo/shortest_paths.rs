pub use crate::ds::graph::Graph;
pub use crate::ds::priority_queue::MinHeap;
use crate::{
    ds::priority_queue::Prioritiness,
    utils::{
        coordinate::{Coordinate, self}, create_adjacency_list_from_files,
        create_id_to_coordinates_hashmap_from_file,
    }, routes::shortestpath::approximate_coordinate, global::Data,
};
use dotenvy::dotenv;
use geoutils::Location;
use rand::Rng;
use rocket::State;
use std::{collections::HashMap, env, error::Error, time::Instant};

const INFINITY: f64 = 9999999.0;

#[derive(Copy, Clone, Debug)]
pub struct NodeInfo {
    pub distance: f64,
    pub id: usize,
}

impl NodeInfo {
    fn new(distance: f64, id: usize) -> Self {
        Self { distance, id }
    }
}

impl Prioritiness for NodeInfo {
    fn priority(&self) -> f64 {
        self.distance
    }

    fn change_priority(&mut self, new_p: f64) {
        self.distance = new_p
    }

    fn id(&self) -> usize {
        self.id
    }
}

pub fn dijkstra(
    g: &Graph,
    src: usize,
    dest: usize,
) -> Result<(f64, Vec<Option<NodeInfo>>), Box<dyn Error>> {
    let mut dist: Vec<f64> = vec![];
    let mut prev = vec![];
    let mut visited = vec![];
    let mut q = MinHeap::new();
    for i in 0..g.edges.len() {
        dist.push(INFINITY);
        prev.push(None);
        visited.push(false);
        if i == src {
            dist[i] = 0.0;
        }
        q.insert(NodeInfo::new(dist[i], i));
    }
    while !q.is_empty() {
        let node = q.extract_min();
        if node.id == dest {
            break;
        }
        visited[node.id] = true;
        for (i, &neighbour) in g.edges[node.id].iter().enumerate() {
            if visited[neighbour] {
                continue;
            }
            let alt = dist[node.id] + g.weights[node.id][i];
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(node);
                q.change_priority(neighbour, alt);
            }
        }
    }
    if dist[dest] == INFINITY {
        return Err("No path found".into());
    }
    Ok((dist[dest], prev))
}

pub fn astar(
    g: &Graph,
    map: &HashMap<usize, Coordinate>,
    src: usize,
    dest: usize,
    heuristic: &dyn Fn(&HashMap<usize, Coordinate>, usize, usize) -> f64,
) -> Result<(f64, Vec<Option<NodeInfo>>), Box<dyn Error>> {
    let mut dist: Vec<f64> = vec![];
    let mut prev = vec![];
    let mut visited = vec![];
    let mut q = MinHeap::new();
    for i in 0..g.edges.len() {
        dist.push(INFINITY);
        prev.push(None);
        visited.push(false);
        if i == src {
            dist[i] = 0.0;
        }
        q.insert(NodeInfo::new(dist[i], i));
    }
    while !q.is_empty() {
        let node = q.extract_min();
        if node.id == dest {
            break;
        }
        visited[node.id] = true;
        for (i, &neighbour) in g.edges[node.id].iter().enumerate() {
            if visited[neighbour] {
                continue;
            }
            let alt = dist[node.id] + g.weights[node.id][i] - heuristic(map, node.id, dest)
                + heuristic(map, neighbour, dest);
            if alt < dist[neighbour] {
                dist[neighbour] = alt;
                prev[neighbour] = Some(node);
                q.change_priority(neighbour, alt);
            }
        }
    }
    if dist[dest] == INFINITY {
        return Err("No path found".into());
    }
    Ok((dist[dest], prev))
}

pub fn harvesine_heuristic(
    map_to_coordinates: &HashMap<usize, Coordinate>,
    src: usize,
    dest: usize,
) -> f64 {
    let src_coord = map_to_coordinates.get(&src).unwrap();
    let dest_coord = map_to_coordinates.get(&dest).unwrap();
    let src_loc = Location::new(src_coord.lat, src_coord.lng);
    let dest_loc = Location::new(dest_coord.lat, dest_coord.lng);
    src_loc.haversine_distance_to(&dest_loc).meters()
}

pub fn reconstruct_path(
    prev: Vec<Option<NodeInfo>>,
    dest: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut path = vec![];
    path.push(dest);
    let mut current = dest;
    while let Some(node) = prev[current] {
        path.push(node.id);
        current = node.id;
    }
    path.reverse();
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 1.0);
        g.add_edge(0, 2, 2.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(1, 3, 3.0);
        g.add_edge(2, 3, 1.0);
        g.add_edge(2, 4, 2.0);
        g.add_edge(3, 4, 1.0);
        let prev = dijkstra(&g, 0, 4).unwrap();
        assert_eq!(prev.0, 4.0);
    }

    #[test]
    fn test_reconstruct_path() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 1.0);
        g.add_edge(0, 2, 2.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(1, 3, 3.0);
        g.add_edge(2, 3, 1.0);
        g.add_edge(2, 4, 2.0);
        g.add_edge(3, 4, 1.0);
        let prev = dijkstra(&g, 0, 4).unwrap();
        let path = reconstruct_path(prev.1, 4).unwrap();
        assert_eq!(path, vec![0, 2, 4]);
    }

    #[test]
    fn test_reconstruct_path2() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1.0);
        g.add_edge(0, 2, 5.0);
        g.add_edge(3, 0, 2.0);
        g.add_edge(1, 2, 2.0);
        let prev = dijkstra(&g, 0, 2).unwrap();
        let path = reconstruct_path(prev.1, 2).unwrap();
        assert_eq!(path, vec![0, 1, 2]);
    }

    #[test]
    #[ignore]
    fn test_dijstra_running_time() {
        dotenv().ok();
        let coordinates_file = env::var("COORDINATES_FILE").unwrap();
        let arcs_file = env::var("ARCS_FILE").unwrap();
        let g = create_adjacency_list_from_files(&coordinates_file, &arcs_file).unwrap();
        let start_point = rand::thread_rng().gen_range(0..g.edges.len());
        let end_point = rand::thread_rng().gen_range(0..g.edges.len());
        let start = Instant::now();
        let prev = dijkstra(&g, start_point, end_point).unwrap();
        let _path = reconstruct_path(prev.1, 1).unwrap();
        println!("Time: {:?}", start.elapsed());
    }

    #[test]
    #[ignore]
    fn test_astart() {
        dotenv().ok();
        let coordinates_file = env::var("COORDINATES_FILE").unwrap();
        let arcs_file = env::var("ARCS_FILE").unwrap();
        let g = create_adjacency_list_from_files(&coordinates_file, &arcs_file).unwrap();
        let map = create_id_to_coordinates_hashmap_from_file(&coordinates_file).unwrap();
        let src = rand::thread_rng().gen_range(0..g.edges.len());
        let dest = rand::thread_rng().gen_range(0..g.edges.len());

        let prev2 = dijkstra(&g, src, dest).unwrap();
        let prev = astar(&g, &map, src, dest, &harvesine_heuristic).unwrap();
        let path = reconstruct_path(prev.1, dest).unwrap();
        let path2 = reconstruct_path(prev2.1, dest).unwrap();
        assert_eq!(path, path2);
    }

    #[test]
    #[ignore = "performance test"]
    fn compare_astar_and_dijkstra_running_times() {
        dotenv().ok();
        let coordinates_file = env::var("COORDINATES_FILE").unwrap();
        let arcs_file = env::var("ARCS_FILE").unwrap();
        let g = create_adjacency_list_from_files(&coordinates_file, &arcs_file).unwrap();
        let map = create_id_to_coordinates_hashmap_from_file(&coordinates_file).unwrap();
        let src = rand::thread_rng().gen_range(0..g.edges.len());
        let dest = rand::thread_rng().gen_range(0..g.edges.len());

        let start = Instant::now();
        let prev = dijkstra(&g, src, dest).unwrap();
        let _path = reconstruct_path(prev.1, dest).unwrap();
        println!("Dijkstra time: {:?}", start.elapsed());

        let start = Instant::now();
        let prev = astar(&g, &map, src, dest, &harvesine_heuristic).unwrap();
        let _path = reconstruct_path(prev.1, dest).unwrap();
        println!("A* time: {:?}", start.elapsed());
    }
}
