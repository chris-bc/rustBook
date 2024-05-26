use rand::Rng;
use std::collections::HashMap;

fn random_vector(count: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for _i in 1..count {
        v.push(rand::thread_rng().gen_range(1..=100));
    }
    v
}

// Requires that v be sorted
// Median is the middle element for an odd number of elements, average of the middle 2 if even
fn calc_median(v: &Vec<i32>) -> i32 {
    let median: i32 = match v.len() % 2 {
        1 => *v.get(1 + v.len() / 2).unwrap_or(&0),
        0 => (v.get(v.len() / 2).unwrap_or(&0) + v.get(1 + v.len() / 2).unwrap_or(&0)) / 2,
        _ => {
            println!("Invalid modulo from v % 2!");
            0
        },
    };
    median
}

fn calc_mode(v: &Vec<i32>) -> i32 {
    // Add vector contents to a HashMap to count them
    let mut items: HashMap<i32, i32> = HashMap::new();
    for i in v {
        let count = items.entry(*i).or_insert(0);
        *count += 1;
    }
    // Find the largest value in the HashMap
    let mut mode = 0;
    let mut mode_count = 0; // There's gotta be a better way than this - Can I take an entire (key, value) pair?
    for (key, value) in &items {
        if *value > mode_count {
            mode_count = *value;
            mode = *key;
        }
    }
    mode
}

fn main() {
    // Create a random vector with 100 elements
    let mut v1 = random_vector(100);
    let mut v2 = random_vector(99);
    
    // Calculate the median
    v1.sort();
    v2.sort();
    println!("v1: {:?}\nv2: {:?}", &v1, &v2);
    let median1 = calc_median(&v1);
    let median2 = calc_median(&v2);
    
    println!("v1 median is {median1}, v2 median is {median2}");

    // HashMap to calculate the mode
    let mode1 = calc_mode(&v1);
    let mode2 = calc_mode(&v2);
    println!("v1 mode is {mode1}, v2 mode is {mode2}");
}
