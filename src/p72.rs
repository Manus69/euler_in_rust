use crate::euler;

const N: u64 = 1_000_000;

pub fn p72()
{
    let phis = euler::phi_table(N + 2);
    println!("{}", phis.iter().sum::<u64>());
}
