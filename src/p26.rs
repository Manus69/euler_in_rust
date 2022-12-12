
use num::Rational64;
use std::collections::HashSet;

fn get_decimal_string(
    fraction: Rational64, 
    string: &mut String, 
    previous: &mut HashSet<Rational64>)
{
    let ratio: i64;
    let remainder: i64;

    if previous.contains(&fraction)
    {
        string.push('*');
        return;
    }

    previous.insert(fraction);

    ratio = fraction.numer() / fraction.denom();
    remainder = fraction.numer() % fraction.denom();

    if ratio >= 1
    {
        string.push_str(ratio.to_string().as_str());
        return get_decimal_string(Rational64::new(remainder * 10, *fraction.denom()), 
                                string, previous);
    }

    string.push('0');

    return get_decimal_string(fraction * 10, string, previous);
}

const N: i64 = 1000;

pub fn p26()
{
    let mut string: String;
    let mut previous: HashSet<Rational64> = HashSet::new();
    let mut max_length: usize = 0;
    let mut denom = 1;

    for k in 1..N
    {
        string = String::new();
        get_decimal_string(Rational64::new(1, k), &mut string, &mut previous);

        if string.len() > max_length
        {
            max_length = string.len();
            denom = k;
        }
    }

    println!("{}", denom);
}