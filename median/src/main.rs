// given a list of integers, use a vector and return the median and mode.
// a hashmap will be helpful.

use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
    println!("The median is {}", median(&v));
    println!("The mode is {}", mode(&v));
}

fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    let middle = v.len() / 2;
    v[middle]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max = (0, 0);
    v.into_iter().for_each(|i| {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > max.1 {
            max = (*i, *count);
        }
    });
    max.0
}
