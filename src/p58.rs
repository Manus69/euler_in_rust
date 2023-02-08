use crate::euler;

const N: usize = 1 << 20;
const THRESHOLD: f64 = 10.0 / 100.0;

fn shell_corner(n: usize, k: usize) -> usize
{
    assert!(k < 4);
    if n == 0 {return 1;}

    return (2 * n + 1) * (2 * n + 1) - 2 * k * n;
}

fn count_primes(n: usize, sieve: &euler::Sieve) -> usize
{
    let mut count: usize = 0;
    
    for k in 0..4
    {
        if euler::is_prime_sieve(shell_corner(n, k) as u64, sieve) {count += 1;}
    }
    
    return count;
}

pub fn p58()
{
    let sieve = euler::sieve(N as u64);
    let mut n_primes: usize = 0;
    let mut total: usize = 1;
    let mut ratio: f64;

    for n in 1..
    {
        n_primes += count_primes(n, &sieve);
        total += 4;
        ratio = n_primes as f64 / total as f64;

        if ratio < THRESHOLD
        {
            println!("{}", 2 * n + 1);
            break;
        }
    }

}