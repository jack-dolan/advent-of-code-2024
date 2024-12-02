use std::fs;

pub fn main() {
    // let data = fs::read_to_string("01/input-test.txt").expect("Unable to read file");
    let data = fs::read_to_string("01/input.txt").expect("Unable to read file");  // REAL DATA
    // println!("{}", data);

    // Make two lists - one of all the left numbers, and one of all the right numbers
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let lines = data.split("\n");
    for line in lines {
        let mut parts = line.split_whitespace();
        vec1.push(parts.next().unwrap().parse::<i32>().unwrap());
        vec2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    // Sort them both
    vec1.sort();
    vec2.sort();

    // println!("{:?}", vec1);
    // println!("{:?}", vec2);

    // For each n'th nuber in both lists, calculate the distance between the two
    // Get the sum of all the distances
    let mut distance_sum = 0;
    let mut similarity_score_sum = 0;

    for i in 0..vec1.len() {
        distance_sum += (vec1[i] - vec2[i]).abs();
        let appearances = vec2.iter().filter(|&n| *n == vec1[i]).count();
        let similarity_score = vec1[i] * appearances as i32;
        similarity_score_sum += similarity_score;
    }

    println!("Sum of distances = {}", distance_sum);
    println!("Sum of similarity scores = {}", similarity_score_sum);

    
}
