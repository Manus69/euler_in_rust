use num::traits::Pow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::euler::{self, Sieve, is_prime_sieve};

const MAX: usize = 1 << 22;

fn concat(lhs: u64, rhs: u64) -> u64
{
    return lhs * (10u64).pow(euler::n_digits(rhs) as u32) + rhs;
}

fn is_concat_prime(p0: u64, p1: u64, sieve: &Sieve) -> bool
{
    let pair = (concat(p0, p1), concat(p1, p0));

    return is_prime_sieve(pair.0, sieve) && is_prime_sieve(pair.1, sieve);
}

fn map_primes(primes: &Vec<u64>, sieve: &Sieve) -> BTreeMap<u64, BTreeSet<u64>>
{
    let mut map = BTreeMap::new();
    let mut set;

    for &p in primes
    {
        set = BTreeSet::<u64>::new();
        for &q in primes
        {
            if is_concat_prime(p, q, sieve)
            {
                set.insert(q);
            }
        }

        if set.len() > 0 {map.insert(p, set);}
    }

    return map;
}

fn _check_compatability(map: &BTreeMap<u64, BTreeSet<u64>>, subset: &Vec<u64>, p: u64) -> bool
{
    let p_compatable = &map[&p];

    for q in subset
    {
        if !p_compatable.contains(q) {return false;}
    }

    return true;
}

fn _find_subset(map: &BTreeMap<u64, BTreeSet<u64>>, mut current_subset: Vec<u64>, keys: &[u64], length: usize) -> Option<Vec<u64>>
{
    let current_added;
    let current_skipped;

    if keys.len() == 0 {return None;}

    if current_subset.len() == length {return Some(current_subset);}

    
    if _check_compatability(map, &current_subset, keys[0]) == false
    {
        return _find_subset(map, current_subset, &keys[1..], length);
    }
    
    current_skipped = _find_subset(map, current_subset.clone(), &keys[1 ..], length);
    current_subset.push(keys[0]);
    current_added = _find_subset(map, current_subset, &keys[1..], length);

    if current_added.is_some() { return current_added;}
    
    return current_skipped;
}

fn find_subset(map: &BTreeMap<u64, BTreeSet<u64>>, length: usize) -> Option<Vec<u64>>
{
    let current: Option<Vec<u64>>;
    let keys = map.keys().map(|x| *x).collect::<Vec<u64>>();

    current = _find_subset(map, Vec::new(), &keys, length);

    return current;
}

pub fn p60()
{
    //this is a hard problem
    let sieve = euler::sieve(MAX);
    let primes = euler::get_primes_less_sieve(10000 as u64, &sieve);

    let map = map_primes(&primes, &sieve);

//    println!("{:? \n}", map);
   println!("{}", map.len());

    // let set = find_subset(&map, 5);
    // println!("{:?}", set);
    // if set.is_some() {println!("{}", set.unwrap().iter().sum::<u64>());}
}