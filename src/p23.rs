use crate::euler;

use std::collections::HashSet;

// const UPPER_BOUND: usize = 50;
const UPPER_BOUND: usize = 28123;

fn get_abundant_numbers(n: u64) -> Vec<u64>
{
    let mut numbers: Vec<u64> = Vec::new();

    for k in 2..=n
    {
        if euler::divisors(k).iter().sum::<u64>() > k
        {
            numbers.push(k);
        }
    }

    return numbers;
}

fn get_sums(numbers: &Vec<u64>) -> HashSet<u64>
{
    let mut sums: HashSet<u64> = HashSet::new();

    for k in 0..numbers.len()
    {
        for j in k..numbers.len()
        {
            sums.insert(numbers[j] + numbers[k]);
        }
    }

    return sums;
}

pub fn p23()
{
    let abundant_numbers = get_abundant_numbers(UPPER_BOUND as u64);
    let numbers: HashSet<u64> = HashSet::from_iter(1..UPPER_BOUND as u64);
    let sums = get_sums(&abundant_numbers);
    
    println!("{:?}", numbers.difference(&sums).sum::<u64>());
}