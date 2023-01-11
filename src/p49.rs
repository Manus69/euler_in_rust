use crate::euler;

const MIN: u64 = 1488;
const MAX: u64 = 9999;
const STEP: u64 = 3330;

fn get_digits(n: u64) -> Vec<u8>
{
    let mut digits: Vec<u8> = (n / 10).to_string().into_bytes();
    digits.sort();

    return digits;
}

fn is_permutation(lhs: u64, rhs: u64) -> bool
{
    return get_digits(lhs) == get_digits(rhs);
}

fn check_prime(n: u64, sieve: &euler::Sieve) -> bool
{
    if n + STEP > MAX {return false;}

    let mut next = n + STEP;
    if !euler::is_prime_sieve(next, sieve) {return false;}
    if !is_permutation(n, next) {return false;}

    next += STEP;

    return next < MAX 
        && euler::is_prime_sieve(next, sieve)
        && is_permutation(n, next);
}

pub fn p49()
{
    let sieve = euler::sieve(MAX);

    for n in MIN..MAX
    {
        if euler::is_prime_sieve(n, &sieve) && check_prime(n, &sieve)
        {
            println!("{}{}{}", n, n + STEP, n + 2 * STEP);
        }
    }
}