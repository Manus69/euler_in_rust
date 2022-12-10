
use std::collections::HashSet;

use crate::euler;

const N: usize = 10_000;

pub fn p21()
{
    let mut friends: HashSet<u64> = HashSet::new();
    let mut sum: u64;
    
    for n in 1..N
    { 
        sum = euler::divisors(n as u64).iter().sum();

        if euler::divisors(sum).iter().sum::<u64>() == n as u64
        {
            if sum != n as u64
            {
                friends.insert(n as u64);
            }
        }
    }
    // println!("{:?}", friends);
    println!("{}", friends.iter().sum::<u64>());
}