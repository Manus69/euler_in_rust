extern crate num_cpus;

use std::thread::{JoinHandle};

const N: u64 = 1_000_000;

fn collatz_length_naive(n: u64) -> u64
{
    let mut n = n;
    let mut length = 1;

    while n > 1
    {
        n = match n % 2 == 0
        {
            true => n / 2,
            false => 3 * n + 1
        };

        length += 1;
    }

    return length;
}

fn compute_max_in_slice(low: u64, high: u64) -> (u64, u64)
{
    let (mut max, mut max_value): (u64, u64) = (0, 0);
    let mut current: u64;

    for n in low..high
    {
        current = collatz_length_naive(n);
        if current > max_value
        {
            max = n;
            max_value = current;
        }
    }

    return (max, max_value);
}

pub fn p14()
{
    let     n_cpus = num_cpus::get();
    let mut threads: Vec<JoinHandle<_>> = Vec::new();
    let mut results: Vec<(u64, u64)> = Vec::new();

    let     step: u64 = N / n_cpus as u64;
    let mut low: u64 = 1;
    let mut high: u64 = low + step;

    for _n in 0..n_cpus
    {
        threads.push(std::thread::spawn(move || compute_max_in_slice(low, high)));
        low = high + 1;
        high += step;
    }

    for t in threads
    {
        results.push(t.join().unwrap());
    }

    println!("{}", results.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().0);
}