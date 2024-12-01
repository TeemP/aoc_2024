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

fn calculate_vector_distance(mut vec_a: Vec<i32>, mut vec_b: Vec<i32>) -> i32 {
    vec_a.sort();
    vec_b.sort();
    vec_a.into_iter()
        .zip(vec_b)
        .map(|(a, b)| (a-b).abs())
        .sum::<i32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (location_list_a, location_list_b) = read_file(&args[1]);
    let distance = calculate_vector_distance(location_list_a, location_list_b);
    println!("Distance of the lists: {}", distance);
}
