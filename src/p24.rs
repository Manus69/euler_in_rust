
use crate::euler;

const N: usize = 1_000_000;

pub fn p24()
{
    let mut numbers: Vec<u32> = (0..10).collect();

    for _k in 0..N - 1
    {
        euler::permute(&mut numbers);
    }

    println!("{:?}", numbers);
}