use crate::euler;

const A: i32 = 1000;
const B: i32 = 1000;

//f(n) = n^2 + an + b
fn eval(n: i32, a: i32, b:i32) -> i32
{
    return n * n + a * n + b;
}

fn is_prime(n: i32, sieve: &Vec<bool>) -> bool
{
    return match n > 0
    {
        true => sieve[n as usize],
        false => false
    }
}

fn count_primes(a: i32, b: i32, sieve: &Vec<bool>) -> usize
{
    let mut count: usize = 0;
    let mut value: i32;

    for n in 0..
    {
        value = eval(n, a, b);
        if is_prime(value, sieve) == true
        {
            count += 1;
        }
        else { break; }
    }

    return count;
}

pub fn p27()
{
    let primes = euler::get_primes_less(B as u64);
    let sieve = euler::sieve((A * B) as u64);
    let (mut max, mut a, mut b): (usize, i32, i32) = (0, 0, 0);
    let mut current: usize;

    for _b in primes
    {
        for _a in -A..b as i32
        {
            current = count_primes(_a, _b as i32, &sieve);
            if current > max
            {
                (max, a, b) = (current, _a, _b as i32);
            }
        }
    }
    
    println!("{}", a * b);
}