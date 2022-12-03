
use crate::euler;

fn test(n: u64) -> u64
{
    return euler::get_primes_less(n).iter().sum();
}

pub fn p10()
{
    println!("{}", test(2_000_000));
}