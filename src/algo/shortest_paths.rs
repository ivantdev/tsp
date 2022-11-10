pub use crate::ds::graph::Graph;
pub use crate::ds::priority_queue::MinHeap;
use crate::ds::priority_queue::Prioritiness;
use std::error::Error;

const INFINITY: f64 = 9999999.0;

#[derive(Copy, Clone, Debug)]
pub struct NodeInfo {
    distance: f64,
    id: usize,
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
}
