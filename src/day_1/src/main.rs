use std::{collections::{HashMap, HashSet}, iter::zip};

mod read_data;

fn get_difference(vec_1: Vec<u32>, vec_2: Vec<u32>) -> u32 {
    zip(vec_1, vec_2).fold(0, |acc, (left, right)| acc + (left).abs_diff(right))
}

fn get_similarity_score(locations: Vec<u32>, occurence_map: HashMap<u32, u32>) -> u32 {
    locations.iter().map(|n| n * occurence_map.get(n).unwrap_or(&0)).sum()
}

fn build_occurence_map(locations: Vec<u32>) -> HashMap<u32, u32> {
    let mut occurence_map = HashMap::<u32, u32>::new();

    locations.iter().for_each(|n| {
        occurence_map.insert(*n, occurence_map.get(n).unwrap_or(&0) + 1);
    });

    return occurence_map
}

fn main() {
    let path = "src/day_1/input/data";
    let (vec_1, vec_2) = read_data::get_problem(path).expect("Unable to read input");
    let occurence_map = build_occurence_map(vec_2);
    let similarity_score = get_similarity_score(vec_1, occurence_map);


    println!("Similarity score is {similarity_score}");
}
