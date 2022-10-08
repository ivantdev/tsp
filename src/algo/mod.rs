#[allow(unused_imports)]
use crate::algo::utils::compare_bfs_runtimes;
use crate::ds::Queue as LinkedQueue;
#[allow(unused_imports)]
use crate::utils::create_adjacency_list_from_files;
use crate::utils::Graph;
use queues::*;

pub mod utils;

pub fn bfs(graph: &Graph) {
    let mut visited = vec![false; graph.edges.len()];
    for i in 0..graph.edges.len() {
        if !visited[i] {
            _bfs(graph, &mut visited, i);
        }
    }
}

fn _bfs(graph: &Graph, visited: &mut Vec<bool>, n: usize) {
    let mut q = LinkedQueue::new();
    q.enqueue(n);
    visited[n] = true;
    while !q.empty() {
        let node = q.dequeue().unwrap();
        for neighbour in &graph.edges[node] {
            if !visited[*neighbour] {
                q.enqueue(*neighbour);
                visited[*neighbour] = true;
            }
        }
    }
}

pub fn bfs_with_array_queue(graph: &Graph) {
    let mut visited = vec![false; graph.edges.len()];
    for i in 0..graph.edges.len() {
        if !visited[i] {
            _bfs_with_array_queue(graph, &mut visited, i);
        }
    }
}

fn _bfs_with_array_queue(graph: &Graph, visited: &mut Vec<bool>, n: usize) {
    let mut q = Queue::new();
    q.add(n).unwrap();
    visited[n] = true;
    while q.size() > 0 {
        let node = q.remove().unwrap();
        for neighbour in &graph.edges[node] {
            if !visited[*neighbour] {
                q.add(*neighbour).unwrap();
                visited[*neighbour] = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let g = create_adjacency_list_from_files("USA-road-d.NY.co", "USA-road-d.NY.gr").unwrap();
        bfs(&g);
    }

    #[test]
    fn test_bfs_with_array_queue() {
        let g = create_adjacency_list_from_files("USA-road-d.NY.co", "USA-road-d.NY.gr").unwrap();
        bfs_with_array_queue(&g);
    }

    #[test]
    #[ignore]
    fn test_compare_bfs_runtimes() {
        let g = create_adjacency_list_from_files("USA-road-d.NY.co", "USA-road-d.NY.gr").unwrap();
        compare_bfs_runtimes(&bfs, &bfs_with_array_queue, &g);
    }
}
