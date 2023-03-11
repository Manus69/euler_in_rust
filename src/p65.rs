use std::{usize};

use num::{BigRational, FromPrimitive};

fn root_two_fraction(_n: usize) -> u64
{
    return 2;
}

//0 1 2 3 4 5 6 7
//1 2 1 1 4 1 1 6
fn e_fraction(n: usize) -> u64
{
    if n == 0 {return 1;}
    if n == 1 {return 2;}

    if (n - 1) % 3 == 0
    {
        return 2 * (n as u64 / 3 + 1);
    }

    return 1;
}

fn sequence_to_rational(length: usize, mut sequence: impl Iterator<Item = u64>) -> BigRational
{
    let fraction;
    let remainder;
    let value;

    if length == 0 {return BigRational::from_i32(0).unwrap();}

    value = sequence.next().unwrap();
    remainder = sequence_to_rational(length - 1, sequence);
    fraction = BigRational::from_u64(value).unwrap() + remainder;

    return fraction.recip();
}

pub fn p65()
{
    let sequence = (0..).map(e_fraction);
    let x = sequence_to_rational(99, sequence) + BigRational::from_i8(2).unwrap();

    println!("{}", x.numer().
                    to_string().
                    as_bytes().
                    iter().
                    map(|c| (*c - b'0') as u64).sum::<u64>());
}
