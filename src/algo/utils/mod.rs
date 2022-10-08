use crate::utils::Graph;
use std::time::Instant;

pub fn compare_bfs_runtimes(f1: &dyn Fn(&Graph), f2: &dyn Fn(&Graph), g: &Graph) {
    let start = Instant::now();
    f1(g);
    let duration = start.elapsed();
    println!("Time elapsed in f1() is: {:?}", duration);

    let start = Instant::now();
    f2(g);
    let duration = start.elapsed();
    println!("Time elapsed in f2() is: {:?}", duration);
}
