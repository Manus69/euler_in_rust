
#[allow(dead_code)]
use std::collections::HashSet;
use std::collections::BTreeSet;
use num;

pub type Sieve = Vec<bool>;

pub fn factor_tuple(n: u64) -> Vec<(u64, usize)>
{
    let mut factor_tuples: Vec<(u64, usize)> = Vec::new();
    let mut p = 2;
    let mut n = n;
    let mut tuple: (u64, usize);

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

pub fn factor_tuple_sieve(n: u64, sieve: & Sieve) -> Vec<(u64, usize)>
{
    if is_prime_sieve(n, sieve) {return vec![(n, 1)];}

    let mut factor_tuple: Vec<(u64, usize)> = Vec::new();
    let mut p = 2;
    let mut n = n;
    let mut power;

    while p <= n
    {
        power = 0;
        while n % p == 0
        {
            power += 1;
            n /= p;
        }

        if power > 0 {factor_tuple.push((p, power));}
        p += 1;
    }

    return factor_tuple;
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

pub fn prime_count_lower_bound(n: usize) -> usize
{
    return (n as f64 / (n as f64).ln()) as usize;
}

pub fn prime_count_upper_bound(n: usize) -> usize
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

pub fn sieve(n: usize) -> Vec<bool>
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

fn _compute_sieve_size(prime_count: usize) -> usize
{
    let mut size = prime_count * 2;

    while prime_count_lower_bound(size) < prime_count
    {
        size *= 2;
    }

    return size;
}

pub fn get_n_primes(n: usize) -> Vec<u64>
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

pub fn get_primes_less_sieve(n: u64, sieve: &Sieve) -> Vec<u64>
{
    let mut primes: Vec<u64> = Vec::new();

    for (index, value) in sieve.iter().enumerate()
    {
        if index >= n as usize { break; }
        if *value == true { primes.push(index as u64); }
    }

    return primes;
}

pub fn get_primes_less(n: u64) -> Vec<u64>
{
    let sieve = sieve(n as usize);

    return get_primes_less_sieve(n, &sieve);
}

pub fn triangular_number(n: u64) -> u64
{
    return n * (n + 1) / 2;
}

pub fn count_divisors(n: u64) -> usize
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
    static VALUES: [u64; 21] =
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 
    39916800, 479001600, 6227020800, 87178291200, 1307674368000, 
    20922789888000, 355687428096000, 6402373705728000, 
    121645100408832000, 2432902008176640000];

    return match n as usize > VALUES.len()
    {
        true => None,
        false => Some(VALUES[n as usize])
    }
}

pub fn n_digits(n: u64) -> u32
{
    // return (n as f64).log10() as usize + 1;
    return n.ilog10() + 1;
}

pub fn is_prime(n: u64) -> bool
{
    let mut p = 2;

    if n < 2 {return false}

    while p * p <= n
    {
        if n % p == 0 {return false}
        p += 1;
    }

    return true;
}

pub fn is_prime_sieve(n: u64, sieve: &Vec<bool>) -> bool
{
    return match n as usize >= sieve.len()
    {
        false => sieve[n as usize],
        _ => is_prime(n)
    }
}

pub fn is_square(n: u64) -> bool
{
    return (n as f64).sqrt().fract() == 0.0;
}

pub fn C(n: usize, k:usize) -> usize
{
    return (0..k).map(|_k| n - _k).fold(1, |acc, item| acc * item);
}

fn _get_index(n: usize, k: usize, choice_index: usize, index_set: &BTreeSet<usize>) -> usize
{
    let n_combinations = C(n, k);
    let frame_size = n_combinations / n;
    let index = (choice_index % n_combinations) / frame_size;

    return *index_set.range(0..).nth(index).unwrap();
}

pub fn choose<T>(set: & [T], n_items: usize, choice_index: usize) -> Vec<T>
where T: Clone
{
    let mut n = set.len();
    let mut result: Vec<T> = Vec::new();
    let mut index_set: BTreeSet<usize> = BTreeSet::from_iter(0..n);
    let mut index;

    for k in (1..n_items + 1).rev()
    {
        index = _get_index(n, k, choice_index, &index_set);
        result.push(set[index].clone());
        index_set.remove(&index);
        n -= 1;
    }

    return result;
}

pub fn phi(n: u64, sieve: Option<& Sieve>) -> usize
{
    let factors = match sieve
    {
        None => factor_tuple(n),
        Some(s) => factor_tuple_sieve(n, s),
    };

    // println!("{:?}", factors);
    return factors.
                iter().
                fold(1, |product, (prime, power)| 
                    product * (prime.pow(*power as u32 - 1) * (prime - 1))) as usize;
}


fn _fill(divisor_map: & mut [u64], index: usize, value: u64)
{
    let mut _index = index << 1;

    while _index < divisor_map.len()
    {
        divisor_map[_index] = value;
        _index += index;
    }
}

fn _sieve_divisor(n: u64) -> Vec<u64>
{
    let mut divisor_map: Vec<u64> = vec![0; n as usize];

    for index in 2..n as usize
    {
        if divisor_map[index] == 0
        {
            _fill(& mut divisor_map, index, index as u64);
        }
    }

    divisor_map[1] = 1;

    return divisor_map;
}


fn _phi_table(n: u64, divisor_map: & Vec<u64>, table: & mut Vec<u64>) -> u64
{
    let divisor = divisor_map[n as usize];
    let remainder;
    let result;
    let gcd;
    let phi_gcd;

    if divisor == 0 {return n - 1;}
    if table[n as usize] != 0 {return table[n as usize];}

    remainder = n / divisor;
    gcd = num::integer::gcd(remainder, divisor);
    phi_gcd = _phi_table(gcd, divisor_map, table);

    result = (_phi_table(divisor, divisor_map, table) * 
            _phi_table(remainder, divisor_map, table) *
            gcd) / phi_gcd;

    table[n as usize] = result;

    return result;
}

pub fn phi_table(n: u64) -> Vec<u64>
{
    let sieve_divisor = _sieve_divisor(n);
    let mut table: Vec<u64> = vec![0; n as usize];

    table[1] = 1;

    return (2..n).map(|_n| _phi_table(_n, & sieve_divisor, & mut table)).collect::<Vec<u64>>();
}