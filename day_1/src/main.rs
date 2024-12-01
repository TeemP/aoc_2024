use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};



fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(path).expect("Unable to read open the path");
    let reader = BufReader::new(file);
    reader.lines()
        .map_while(Result::ok)
        .map(|s|{
            let c = s.split(" ")
            .filter_map(|p| p.parse().ok())
            .collect::<Vec<i32>>();
            if c.len() == 2 {
                (c[0],c[1])
            } else {
                panic!("File should have pairs of numbers separated by one or more spaces")
            }
        })
        .unzip()
}

fn calculate_vector_distance_by_abs(vec_a: &mut [i32], vec_b: &mut [i32]) -> i32 {
    vec_a.sort();
    vec_b.sort();
    vec_a.iter_mut()
        .zip(vec_b)
        .map(|(a, b)| (*a-*b).abs())
        .sum::<i32>()
}

fn calculate_vector_distance_by_count(vec_a: &mut [i32], vec_b: &mut [i32]) -> i32 {
    vec_a.sort();
    vec_b.sort();
    vec_a.iter_mut()
        .map(|a| {
            *a*(vec_b.iter_mut()
            .filter(|v| **v==*a)
            .count() as i32)
        })
        .sum::<i32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mut location_list_a, mut location_list_b) = read_file(&args[1]);
    let distance = calculate_vector_distance_by_abs(&mut location_list_a, &mut location_list_b);
    println!("Distance of the lists part 1: {}", distance);
    let distance = calculate_vector_distance_by_count(&mut location_list_a, &mut location_list_b);
    println!("Distance of the lists part 2: {:?}", distance);
}
