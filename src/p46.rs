use crate::euler;

const N: usize = 1 << 15;
const START: usize = 35;

fn is_repr(n: usize, sieve: &Vec<bool>) -> bool
{
    let mut twice_product: usize;

    for m in 1..
    {
        twice_product = 2 * m * m;
        if twice_product >= n
        {
            return false;
        }

        if euler::is_prime_sieve((n - twice_product) as u64, sieve)
        {
            return true;
        }
    }

    return false;
}

pub fn p46()
{
    let sieve = euler::sieve(N as u64);

    for n in START..
    {
        if n % 2 == 0 || euler::is_prime_sieve(n as u64, &sieve)
        {
            continue;
        }

        if !is_repr(n, &sieve)
        {
            println!("{}", n);
            break;
        }
    }

}