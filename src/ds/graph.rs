#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<Vec<usize>>,
    pub weights: Vec<Vec<u32>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
            weights: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: u32) {
        self.edges[u].push(v);
        self.weights[u].push(w);
    }
}
