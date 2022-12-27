
use crate::euler;

const MAX: u64 = 1_000_000;

fn digit_factorial(n: u64) -> Option<u64>
{
    return n.to_string().as_bytes().iter().map(|d| euler::factorial((*d - b'0') as u64)).sum();
}

pub fn p34()
{
    let mut sum: u64 = 0;

    for k in 3..MAX
    {
        if digit_factorial(k).unwrap() == k
        {
            sum += k;
        }
    }

    println!("{}", sum);
}