use crate::euler;

const MAX: u64 = 1_000_000;
const BASE: u64 = 10;

fn rotate_digits(n: u64) -> Option<u64>
{
    let n_digits = euler::n_digits(n);
    let base = BASE.pow(n_digits as u32 - 1);
    let digit = n / base;
    let result = (n - digit * base) * BASE + digit;

    return match euler::n_digits(result) == n_digits
    {
        true => Some(result),
        _ => None
    };
}

fn check_rotations(p: u64, sieve: &Vec<bool>) -> bool
{
    let n_rotations = euler::n_digits(p) - 1;
    let mut current: Option<u64> = Some(p);

    for _k in 0..n_rotations
    {
        current = rotate_digits(current.unwrap());
        if current.is_none() {return false;}

        if sieve[current.unwrap() as usize] == false {return false;}
    }

    return true;
}

pub fn p35()
{
    let     sieve = euler::sieve(MAX);
    let mut sum: usize = 0;

    for (index, value) in sieve.iter().enumerate()
    {
        if *value == true
        {
            if check_rotations(index as u64, &sieve) == true {sum += 1;}
        }
    }

    println!("{}", sum);
}