use std::collections::HashSet;
use crate::euler;

fn factorize(bot: u64, top: u64) -> Vec<(u64, u64)>
{
    return euler::factor_tuple(bot).
            iter().
            map(|(x, y)| (*x, y * top)).
            collect::<Vec<(u64, u64)>>();
}

const LOW: u64 = 2;
const HIGH: u64 = 100;

pub fn p29()
{
    let mut unique: HashSet<Vec<(u64, u64)>> = HashSet::new();
    let mut factors: Vec<(u64, u64)>;

    for bot in LOW..=HIGH
    {
        for top in LOW..=HIGH
        {
            factors = factorize(bot, top);

            if !unique.contains(&factors) { unique.insert(factors); }
        }
    }

    println!("{}", unique.len());
}
