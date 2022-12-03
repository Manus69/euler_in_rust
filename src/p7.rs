use crate::euler;

pub fn p7(n: u64) -> u64
{
    return *euler::get_n_primes(n).last().unwrap();
}