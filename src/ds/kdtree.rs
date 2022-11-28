use std::{ops::{Add, Sub, Mul}, fmt, cmp::Ordering};
use pdqselect::select_by;

#[derive(Debug, Clone)]
pub struct KdTree<T> where T: Copy + PartialEq, {
    pub root: Vec<Vec<T>>,
}

impl<T> KdTree<T> where T: Copy + PartialEq + PartialOrd + fmt::Debug + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
    pub fn new(mut arr: Vec<Vec<T>>) -> Self {
        Self::sort_kdtree(&mut arr, 0);
        Self {
            root: arr.to_owned(),
        }
    }

    pub fn closest<'a>(target:&'a Vec<T>, point_a: &'a Vec<T>, point_b: &'a Vec<T>) -> Vec<T> {
        let target_a = Self::distance(target, point_a);
        let target_b = Self::distance(target, point_b);

        if target_a < target_b { point_a.clone() }
        else { point_b.clone() }
    }

    pub fn distance<'a>(target: &'a Vec<T>, point: &'a Vec<T>) -> T {
        let x: T = (target[0] - point[0])*(target[0] - point[0]);
        let y: T = (target[1] - point[1])*(target[1] - point[1]);
        x+y
    }

    pub fn compare(a: &Vec<T>, b: &Vec<T>, k: usize) -> Ordering {
        if a[k] < b[k] { Ordering::Less }
        else { Ordering::Greater }
    }

    pub fn sort_kdtree(array: &mut [Vec<T>], k: usize) {
        if array.len() > 1 {
            select_by(array, array.len()/2, |a , b| Self::compare(&a, &b, k));
            let size = array.len();
            let middle: usize = if size%2 != 0 { size/2 } else { size/2 - 1 };

            Self::sort_kdtree(&mut array[..middle], (k+1)%2);
            Self::sort_kdtree(&mut array[middle + 1..], (k+1)%2);
        }
    }

    pub fn get_nearest_neighbor(&self, target: &Vec<T>) -> Result<Vec<T>, ()> {
        self.nearest_neighbor(&self.root, target, 0)
    }

    pub fn nearest_neighbor(&self, arr: &[Vec<T>], target: &Vec<T>, k: usize) -> Result<Vec<T>, ()> {
        if arr.len() == 1 { return Ok(arr[0].clone()) }

        let size = arr.len();
        let middle: usize = if size%2 != 0 { size/2 } else { size/2 - 1 };
        let first: &[Vec<T>];
        let second: &[Vec<T>];
        if target[k] < arr[middle][k] {
            first = &arr[..middle];
            second = &arr[middle + 1..];
        } else {
            first = &arr[middle + 1..];
            second = &arr[..middle];
        }

        let mut option: Vec<T> = if first.len() > 0 { self.nearest_neighbor(first, target, (k+1)%2).unwrap() } else { arr[middle].clone() };
        let mut best: Vec<T> = Self::closest(target, &arr[middle], &option);

        let r: T = Self::distance(target, &best);
        let rc: T = target[k] - arr[middle][k];

        if r >= (rc*rc) {
            option =  if second.len() > 0 { self.nearest_neighbor(second, target, (k+1)%2).unwrap() } else { best.clone() };
            best = Self::closest(target, &option, &best);
        }

        Ok(best)
    }

    pub fn get_n_nearest_neighbor(&self, target: &Vec<T>, n: usize) -> Result<Vec<Vec<T>>, ()> {
        let mut bests: Vec<(Vec<T>, T)> = Vec::with_capacity(n);
        let _ = self.n_nearest_neighbor(&self.root, target, 0, &mut bests);
        bests.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let results = bests.iter().map(|x| x.0.to_owned()).collect::<Vec<_>>();
        Ok(results)
    }

    pub fn save_best(&self, best: &Vec<T>, r: T, bests: &mut Vec<(Vec<T>, T)>) {
        for i in 0..bests.len() {
            if bests[i].0[2] == best[2] {
                return;
            } else {
            }
        }
        if bests.len() < bests.capacity() {
            let tuple = (best.clone(), r.clone());
            bests.push(tuple.clone());
        } else {
            bests.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            bests.reverse();
            for i in 0..bests.len() {
                if bests[i].1 > r {
                    bests[i] = (best.clone(), r.clone());
                    break;
                }
            }
        }
    }

    pub fn n_nearest_neighbor(&self, arr: &[Vec<T>], target: &Vec<T>, k: usize, bests: &mut Vec<(Vec<T>, T)>) -> Result<Vec<T>, ()> {
        if arr.len() == 1 { 
            let best = arr[0].clone();
            let r = Self::distance(target, &arr[0]);
            self.save_best(&best, r, bests);
            return Ok(arr[0].clone())
        }


        let size = arr.len();
        let middle: usize = if size%2 != 0 { size/2 } else { size/2 - 1 };
        let first: &[Vec<T>];
        let second: &[Vec<T>];
        if target[k] < arr[middle][k] {
            first = &arr[..middle];
            second = &arr[middle + 1..];
        } else {
            first = &arr[middle + 1..];
            second = &arr[..middle];
        }
        
        let r: T = Self::distance(target, &arr[middle]);
        self.save_best(&arr[middle], r, bests);

        let mut option: Vec<T> = if first.len() > 0 { self.n_nearest_neighbor(first, target, (k+1)%2, bests).unwrap() } else { arr[middle].clone() };
        let r: T = Self::distance(target, &option);
        self.save_best(&option, r, bests);

        let mut best: Vec<T> = Self::closest(target, &arr[middle], &option);
        let r: T = Self::distance(target, &best);
        self.save_best(&best, r, bests);
        
        let rc: T = target[k] - arr[middle][k];
        

        if r >= (rc*rc) {
            option =  if second.len() > 0 { self.n_nearest_neighbor(second, target, (k+1)%2, bests).unwrap() } else { best.clone() };
            let r: T = Self::distance(target, &option);
            self.save_best(&option, r, bests);

            best = Self::closest(target, &option, &best);
            let r: T = Self::distance(target, &best);
            self.save_best(&best, r, bests);
        }

        Ok(best)
    }

}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::utils::create_kd_tree_from_file;
    use dotenvy::dotenv;
    use std::{env};

    #[test]
    fn test_n_nearest_neighbor_kd_tree() {
        dotenv().ok();

        let coordinates = env::var("COORDINATES_FILE").unwrap();
        let kd_tree = create_kd_tree_from_file(&coordinates).unwrap();

        let _result = kd_tree.get_nearest_neighbor(&[4.665179, -74.063324].to_vec()).unwrap();
        let results = kd_tree.get_n_nearest_neighbor(&[4.665179, -74.063324].to_vec(), 100).unwrap();
        
        
        println!("# results: {:?}", results.len());
        for i in 0..results.len() {
            println!("{:?}", results[i]);
        }
    }

}