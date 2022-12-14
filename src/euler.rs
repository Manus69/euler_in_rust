
#[allow(dead_code)]
use std::collections::HashSet;
use std::vec;

use num;

pub fn factor_tuple(n: u64) -> Vec<(u64, u64)>
{
    let mut factor_tuples: Vec<(u64, u64)> = Vec::new();
    let mut p = 2;
    let mut n = n;
    let mut tuple: (u64, u64);

    while p <= n
    {
        tuple = (p, 0);
        while n % p == 0
        {
            tuple.1 += 1;
            n = n / p;
        }

        if tuple.1 > 0 { factor_tuples.push(tuple); }
        p += 1;
    }

    return factor_tuples;
}

pub fn factor(n: u64) -> Vec<u64>
{
    let mut factors: Vec<u64> = Vec::new();
    let mut max = n;
    let mut p = 2;
    
    while p <= max
    {
        while max % p == 0
        {
            factors.push(p);
            max /= p;
        }

        p += 1;
    }

    return factors;
}

pub fn gcd(numbers: &[u64]) -> u64
{
    let _gcd: u64;
    let _rhs_gcd: u64;

    if numbers.len() == 0 { return 0; }
    if numbers.len() == 1 { return numbers[0]; }

    _gcd = num::integer::gcd(numbers[0], numbers[1]);

    if numbers.len() == 2 { return _gcd; }

    _rhs_gcd = gcd(&numbers[2..]);

    return num::integer::gcd(_gcd, _rhs_gcd);
}

pub fn lcm(numbers: &[u64]) -> u64
{
    let _lcm: u64;
    let _rhs_lcm: u64;

    if numbers.len() == 0 { return 0; }
    if numbers.len() == 1 { return numbers[0]; }

    _lcm = num::integer::lcm(numbers[0], numbers[1]);

    if numbers.len() == 2 { return _lcm; }

    _rhs_lcm = lcm(&numbers[2..]);

    return num::integer::lcm(_lcm, _rhs_lcm);
}

pub fn prime_count_lower_bound(n: u64) -> u64
{
    return (n as f64 / (n as f64).ln()) as u64;
}

pub fn prime_count_upper_bound(n: u64) -> u64
{
    return 2 * prime_count_lower_bound(n);
}

fn _remove_multiples(numbers: &mut [bool], step: usize)
{
    let mut index: usize = step;

    while index < numbers.len()
    {
        numbers[index] = false;
        index += step;
    }
}

pub fn sieve(n: u64) -> Vec<bool>
{
    assert!(n > 2);

    let mut _sieve = vec![true; n as usize + 1];
    let mut p: usize = 2;

    _sieve[0] = false;
    _sieve[1] = false;

    while p <= n as usize
    {
        if _sieve[p] == true
        {
            _remove_multiples(&mut _sieve[p..], p);
        }

        p += 1;
    }

    return _sieve;
}

fn _compute_sieve_size(prime_count: u64) -> u64
{
    let mut size = prime_count * 2;

    while prime_count_lower_bound(size) < prime_count
    {
        size *= 2;
    }

    return size;
}

pub fn get_n_primes(n: u64) -> Vec<u64>
{
    let     size = _compute_sieve_size(n);
    let     sieve = sieve(size);
    let mut primes = Vec::new();

    for (index, value) in sieve.iter().enumerate()
    {
        if primes.len() >= n as usize { break; }
        if *value == true { primes.push(index as u64); }
    }

    return primes;
}

pub fn get_primes_less(n: u64) -> Vec<u64>
{
    let     sieve = sieve(n);
    let mut primes: Vec<u64> = Vec::new();

    for (index, value) in sieve.iter().enumerate()
    {
        if *value == true { primes.push(index as u64); }
    }

    return primes;
}

pub fn triangular_number(n: u64) -> u64
{
    return n * (n + 1) / 2;
}

pub fn count_divisors(n: u64) -> u64
{
    let     factors = factor_tuple(n);
    let mut count = 1;

    for tuple in factors
    {
        count *= tuple.1 + 1;
    }

    return count;
}

pub fn divisors(n: u64) -> HashSet<u64>
{
    assert!(n != 0);

    let mut set = HashSet::new();
    let mut d: u64 = 2;

    set.insert(1);

    while d * d <= n
    {
        if n % d == 0
        {
            set.insert(d);
            set.insert(n / d);
        }

        d += 1;
    }

    return set;
}

fn _find_index<T>(vector: &Vec<T>) -> Option<usize>
where T: Ord
{
    for k in (1..vector.len()).rev()
    {
        if vector[k - 1] < vector[k] { return Some(k - 1);}
    }
    
    return None;
}

fn _find_first_greater<T>(vector: &Vec<T>, index: usize) -> Option<usize>
where T: Ord
{
    for k in (index..vector.len()).rev()
    {
        if vector[k] >= vector[index] { return Some(k);}
    }

    return None;
}

pub fn permute<T>(vector: &mut Vec<T>) -> bool
where T: Ord
{
    if vector.len() < 2 { return false; }

    let index = _find_index(vector);
    if index.is_none()
    {
        vector.reverse();
        return false;
    }

    let index = index.unwrap();
    let right_index = _find_first_greater(vector, index).unwrap();

    vector.swap(index, right_index);
    vector[index + 1..].reverse();

    return true;
}

pub fn factorial(n: u64) -> Option<u64>
{
    return match n
    {
        0 =>  Some(1),
        1 =>  Some(1),
        2 =>  Some(2),
        3 =>  Some(6),
        4 =>  Some(24),
        5 =>  Some(120),
        6 =>  Some(720),
        7 =>  Some(5040),
        8 =>  Some(40320),
        9 =>  Some(362880),
        10 => Some(3628800),
        11 => Some(39916800),
        12 => Some(479001600),
        13 => Some(6227020800),
        14 => Some(87178291200),
        15 => Some(1307674368000),
        16 => Some(20922789888000),
        17 => Some(355687428096000),
        18 => Some(6402373705728000),
        19 => Some(121645100408832000),
        20 => Some(2432902008176640000),
        _ => None
    }
}