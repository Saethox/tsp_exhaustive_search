extern crate core;

use std::io::Cursor;
use std::ops::{Index, IndexMut};
use std::time::Instant;

use itertools::Itertools;
use tsplib::NodeCoord;

struct SquareMatrix<T> {
    storage: Vec<Vec<T>>,
}

impl<T: Default> SquareMatrix<T> {
    pub fn with_defaults(size: usize) -> Self {
        Self {
            storage: (0..size).into_iter().map(|_| (0..size).into_iter().map(|_| Default::default()).collect()).collect()
        }
    }
}

impl<T> Index<(usize, usize)> for SquareMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.storage[index.0 - 1][index.1 - 1]
    }
}

impl<T> IndexMut<(usize, usize)> for SquareMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut (&mut self.storage[index.0 - 1])[index.1 - 1]
    }
}

type DistanceFunction = fn([f32; 2], [f32; 2]) -> f32;

impl SquareMatrix<u64> {
    pub fn distance_matrix(coords: &[(usize, f32, f32)], measure: DistanceFunction) -> Self {
        let n = coords.len();
        let mut matrix = SquareMatrix::with_defaults(n);

        for i in 1..n {
            for j in 1..n {
                matrix[(i, j)] = measure([coords[i].1, coords[i].2], [coords[j].1, coords[j].2]) as u64;
            }
        }

        matrix
    }

    pub fn route_length(&self, route: &[usize]) -> u64 {
        route.iter().circular_tuple_windows::<(_, _)>().map(|(&u, &v)| self[(u, v)]).sum()
    }
}

#[allow(dead_code)]
fn euclidean(a: [f32; 2], b: [f32; 2]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

fn coordinate_to_radian(coordinate: f32) -> f32 {
    let degrees = coordinate.floor();
    let minutes = coordinate - degrees;
    std::f32::consts::PI * (degrees + 5.0 * minutes / 3.0) / 180.0
}

fn geographical(a: [f32; 2], b: [f32; 2]) -> f32 {
    let lat_a = coordinate_to_radian(a[0]);
    let lng_a = coordinate_to_radian(a[1]);

    let lat_b = coordinate_to_radian(b[0]);
    let lng_b = coordinate_to_radian(b[1]);

    let radius = 6378.388;

    let q1 = (lng_a - lng_b).cos();
    let q2 = (lat_a - lat_b).cos();
    let q3 = (lat_a + lat_b).cos();
    radius * (0.5 * ((1. + q1) * q2 - (1. - q1) * q3)).acos() + 1.
}

fn main() {
    let data = include_str!("tsp/burma14.tsp");
    let problem = tsplib::parse(Cursor::new(data)).unwrap();
    let coords = match problem.node_coord.unwrap() {
        NodeCoord::Two(coords) => coords,
        _ => panic!(),
    };

    let distances = SquareMatrix::distance_matrix(&coords, geographical);
    let nodes: Vec<_> = coords.iter().map(|c| c.0).collect();
    let n = nodes.len();

    let before = Instant::now();

    let minimum_route = nodes
        .into_iter()
        .permutations(n)
        // .take(1e8 as usize)
        // .par_bridge()
        .min_by_key(|route| {
            distances.route_length(route)
        })
        .unwrap();

    println!("Minimum Route: {:?}", minimum_route);
    println!("Distance: {}", distances.route_length(&minimum_route));
    println!("Time Taken: {:?}", before.elapsed());
}
