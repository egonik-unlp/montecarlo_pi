use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::env::args;
const RUNS: usize = 1e9 as usize;

fn main() {
    let repeats : usize = args().nth(1).unwrap_or_else(|| format!("{RUNS}")).parse().unwrap();
    let rng = thread_rng();
    let dist = Uniform::from(0f32..1f32);
    let xs = dist.sample_iter(rng.clone()).take(repeats);
    let ys = dist.sample_iter(rng).take(repeats);
    let good_ones = xs
        .into_iter()
        .zip(ys.into_iter())
        .filter(|(x, y)| x.powi(2) + y.powi(2) <= 1f32)
        .count();
    let pi = (good_ones * 4) as f32 / repeats as f32;
    println!("Pi = {} with {repeats} repetitions", pi);
    
}
