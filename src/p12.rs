use crate::euler;

fn p12_test(n_factors: u64) -> u64
{
    let mut n: u64 = 1;
    let mut count: u64;
    let mut number;

    loop
    {
        number = euler::triangular_number(n);
        count = euler::count_divisors(number);
        n += 1;

        if count > n_factors { break; }
    }

    return number;
}

pub fn p12()
{
    println!("{}", p12_test(500));
}