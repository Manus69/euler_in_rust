
#[allow(dead_code)]

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

pub fn divisors(n: u64) -> Vec<u64>
{
    assert!(n != 0);

    let mut divisors: Vec<u64> = Vec::new();
    let mut d: u64 = 2;

    if n == 1 { return vec![1]; }

    divisors.push(1);
    divisors.push(n);
    
    while d * d < n
    {
        if n % d == 0 
        {
            divisors.push(d);
            divisors.push(n / d);
        }
        
        d += 1;
    }

    return divisors;
}