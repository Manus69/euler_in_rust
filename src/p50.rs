use crate::euler;

const MAX: usize = 1_000_000;

fn find_prime_sum(primes: &Vec<u64>, sieve: &Vec<bool>, frame_size: usize) -> Option<u64>
{
    let mut sum: u64;
    let mut result: Option<u64> = None;

    for frame in primes.windows(frame_size)
    {
        sum = frame.iter().sum();
        if sum as usize> MAX {break;};

        if euler::is_prime_sieve(sum, sieve)
        {
            result = Some(sum);
        }
    }

    return result;
}

pub fn p50()
{
    let sieve = euler::sieve(MAX as u64);
    let primes = euler::get_primes_less_sieve(MAX as u64, &sieve);

    let mut result;

    for frame_size in (1..1000).rev()
    {
        result = find_prime_sum(&primes, &sieve, frame_size);

        if result.is_some()
        {
            println!("{}", result.unwrap());
            break;
        }
    }
}