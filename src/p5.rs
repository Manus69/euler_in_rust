
use crate::euler;

pub fn p5(numbers: std::ops::Range<u64>) -> u64
{
    let v: Vec<u64> = numbers.collect();

    return euler::lcm(&v);
}