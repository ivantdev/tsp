use crate::{
    ds::priority_queue::{MinHeap, Prioritiness},
    utils::{coordinate::Coordinate, create_kd_tree_from_file},
};
use std::{
    fs,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
struct Distance {
    pub distance: f64,
    pub _coordinate: Coordinate,
    id: usize,
}

impl Prioritiness for Distance {
    fn priority(&self) -> f64 {
        self.distance
    }
    fn id(&self) -> usize {
        self.id
    }
    fn change_priority(&mut self, p: f64) {
        self.distance = p;
    }
}

pub fn compare_coordinates_approximation() {
    // 1 Approximate coordinate with kd-tree
    let nodes_file = "nodes_colombia.txt";
    let test_file = "test.colombia";

    let mut tests = vec![];
    for line in fs::read_to_string(test_file).unwrap().lines() {
        let mut split = line.split_whitespace();
        let lat = split.next().unwrap().parse::<f64>().unwrap();
        let lng = split.next().unwrap().parse::<f64>().unwrap();
        let coordinate = Coordinate { lat, lng, id: 0 };
        tests.push(coordinate);
    }

    let nodes_file_s = fs::read_to_string(nodes_file).unwrap();

    let mut coordinates = vec![];

    for line in nodes_file_s.lines() {
        let mut split_line = line.split_whitespace();

        let _id = split_line.next();

        let latitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();
        let longitude = split_line
            .next()
            .unwrap()
            .to_string()
            .parse::<f64>()
            .unwrap();

        let coordinate = Coordinate {
            lat: latitude,
            lng: longitude,
            id: 0,
        };

        coordinates.push(coordinate);
    }

    let mut times_elapsed = vec![];
    let start = Instant::now();
    let kd_tree = create_kd_tree_from_file(&(nodes_file.to_string())).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed creating kd-tree: {:?}", duration);
    for test_coordinate in tests.iter() {
        let start = Instant::now();
        let _nearest_coordinate = kd_tree
            .nearest_neighbor(&kd_tree.root, &vec![test_coordinate.lat, test_coordinate.lng], 0)
            .unwrap();
        let duration = start.elapsed();
        times_elapsed.push(duration);
    }
    println!(
        "Average time elapsed searching with kd-tree: {:?}",
        average_time(&times_elapsed)
    );
    println!(
        "Worst time elapsed searching with kd-tree: {:?}",
        worst_time(&times_elapsed)
    );
    println!("--------------------------------------------------");

    // 2 Approximate coordinate with array

    let mut min_distance = f64::MAX;
    let mut _nearest_coordinate = Coordinate { lat: 0.0, lng: 0.0, id: 0 };
    let mut times_elapsed = vec![];
    for test_coordinate in tests.iter() {
        let start = Instant::now();
        for coordinate in &coordinates {
            let distance = euclidean_distance(&test_coordinate, &coordinate);
            if distance < min_distance {
                min_distance = distance;
                _nearest_coordinate = *coordinate;
            }
        }
        let duration = start.elapsed();
        times_elapsed.push(duration);
    }
    println!(
        "Average time elapsed searching with array: {:?}",
        average_time(&times_elapsed)
    );
    println!(
        "Worst time elapsed searching with array: {:?}",
        worst_time(&times_elapsed)
    );
    println!("--------------------------------------------------");

    // 3 Approximate coordinate with min heap

    let mut times_elapsed = vec![];
    for test_coordinate in tests.iter() {
        let start = Instant::now();
        let mut min_heap = MinHeap::new();
        for (i, coordinate) in coordinates.iter().enumerate() {
            let distance = euclidean_distance(&test_coordinate, &coordinate);
            min_heap.insert(Distance {
                distance,
                _coordinate: *coordinate,
                id: i,
            });
        }
        let duration = start.elapsed();
        times_elapsed.push(duration);
    }
    println!(
        "Average time elapsed searching with min heap: {:?}",
        average_time(&times_elapsed)
    );
    println!(
        "Worst time elapsed searching with min heap: {:?}",
        worst_time(&times_elapsed)
    );
    println!("--------------------------------------------------");
}

fn euclidean_distance(coordinate1: &Coordinate, coordinate2: &Coordinate) -> f64 {
    let lat1 = coordinate1.lat;
    let lng1 = coordinate1.lng;
    let lat2 = coordinate2.lat;
    let lng2 = coordinate2.lng;

    let lat_diff = lat1 - lat2;
    let lng_diff = lng1 - lng2;

    let lat_diff_squared = lat_diff.powi(2);
    let lng_diff_squared = lng_diff.powi(2);

    let sum = lat_diff_squared + lng_diff_squared;

    sum.sqrt()
}

fn average_time(times_elapsed: &Vec<Duration>) -> Duration {
    let mut sum = Duration::new(0, 0);
    for duration in times_elapsed {
        sum = sum + *duration;
    }
    sum.checked_div(times_elapsed.len() as u32).unwrap()
}

fn worst_time(times_elapsed: &Vec<Duration>) -> Duration {
    let mut worst_time = Duration::new(0, 0);
    for duration in times_elapsed {
        if *duration > worst_time {
            worst_time = *duration;
        }
    }
    worst_time
}
