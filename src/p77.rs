use std::collections::{BTreeSet, HashMap};
use crate::euler;
use crate::p76;

const N_PRIMES: usize = 1 << 7;

fn find_first(limit: usize) -> usize
{
    let numbers: BTreeSet<usize> = euler::get_n_primes(N_PRIMES).
                                        iter().
                                        map(|p| *p as usize).
                                        collect::<BTreeSet<usize>>(); 
    
    let mut table: HashMap<(usize, BTreeSet<usize>), usize> = HashMap::new();

    for n in 2..
    {
        if p76::n_ways(n, numbers.clone(), &mut table) > limit {return n;}
    }

    return 0;
}

pub fn p77()
{
    let n = find_first(5000);

    println!("{}", n);
}