use std::collections::{hash_set, HashSet};
use crate::euler;

const PATH: &str = "./src/p042_words.txt";

fn get_triangular_set(size: usize) -> HashSet<u64>
{
    return (1..size)
        .map(|n| euler::triangular_number(n as u64))
        .collect();
}

fn is_triangular(word: &str, t_set: &HashSet<u64>) -> bool
{
    let result: u64 = word.as_bytes()
                    .iter()
                    .map(|b| b - b'A' + 1)
                    .fold(0u64, |acc, b| acc + b as u64);
    
    return t_set.contains(&result);
}

const N: usize = 20;

pub fn p42()
{
    let words = std::fs::read_to_string(PATH).
        ok().
        unwrap();
    
    let words: Vec<&str> = words.
        split(',').
        map(|_str| _str.trim_matches('"')).
        collect();

    let t_set = get_triangular_set(20);
    let count = words
                .iter()
                .filter(|w| is_triangular(w, &t_set))
                .count();
    
    println!("{}", count);
}