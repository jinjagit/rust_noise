use libnoise::*;
use std::time::Instant;
use argminmax::ArgMinMax;

fn main() {
    // res 512 (+ border 3) = 268_324 points
    // res 1024 (+ border 3) = 1_060_900 points
    // res 2048 (+ border 3) = 4_218_916 points
    let mut vec_of_vec3s: Vec<Vec<f64>> = vec![];

    for _i in 0..4218916 {
        vec_of_vec3s.push(vec![1.1 + _i as f64, 2.2, 3.3]);
    }

    let start_time = Instant::now();

    // let mut noise_data: Vec<f64> = Vec::new();

    let octaves = 6;
    let frequency = 1.0;
    let lacunarity = 2.0;
    let persistence = 0.5;

    // build a simplex noise generator seeded with 42
    let generator = Source::simplex(42).fbm(octaves, frequency, lacunarity, persistence);

    // Slightly faster than for loop
    let result = vec_of_vec3s.iter().map(|x| generator.sample([x[0], x[1], x[2]])).collect::<Vec<_>>();

    let elapsed = start_time.elapsed();
    println!("{:?}", result[42]);
    println!("libnoise time: {:?}", elapsed);

    let start_minmax = Instant::now();
    let (min, max) = result.argminmax();  // apply extension

    let elapsed_minmax = start_minmax.elapsed();
    println!("min: {}, max: {}", min, max);
    println!("result[min]: {}, result[max]: {}", result[min], result[max]);
    println!("minmax time: {:?}", elapsed_minmax);
    // TODO: Add other noise library examples for comparison
}

