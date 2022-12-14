
use num::{BigUint, FromPrimitive};

fn compute(n: usize) -> BigUint
{
    if n % 2 == 1 || n == 0 { return BigUint::from_u64(0).unwrap(); }

    let mut l_shape:    BigUint = BigUint::from_u8(1).unwrap();
    let mut rect:       BigUint = BigUint::from_u8(3).unwrap();
    let mut current:    BigUint;

    for _k in (2..n).step_by(2)
    {
        current = 3 as u8 * &rect + 2 as u8 * &l_shape;
        l_shape = &rect + &l_shape;
        rect = current;
    }

    return rect;
}

pub fn tiling(n: usize)
{
    let n_ways = compute(n);
    println!("{}", n_ways);
    // println!("{}", n_ways.to_string().len());
}