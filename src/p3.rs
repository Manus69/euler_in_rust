use crate::euler::get_prime_factors;


pub fn p3(n: u64) -> u64
{
    let factors = get_prime_factors(n);

    return *factors.last().unwrap();
}