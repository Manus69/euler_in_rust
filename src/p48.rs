
const LIMIT: u64 = 1000;

fn add_mod(lhs: u64, rhs: u64, m: u64) -> u64
{
    return lhs % m + rhs % m;
}

fn pow_mod(base: u64, pow: u64, m: u64) -> u64
{
    let mut result = 1;

    for _exp in 0..pow
    {
        result = (result % m) * base; 
    }

    return result % m;
}

pub fn p48()
{
    let mut pow: u64;
    let mut sum: u64 = 0;
    let m: u64 = 10u64.pow(10 as u32);

    for n in 1..=LIMIT
    {
        // pow = n.wrapping_pow(n);
        pow = pow_mod(n, n, m);
        sum = sum % m + pow;
    }

    println!("{}", sum % m);
}