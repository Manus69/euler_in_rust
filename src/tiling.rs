
use num::{BigUint, FromPrimitive};
fn compute(n: usize) -> BigUint
{
    if n % 2 == 1 || n == 0 { return BigUint::from_u64(0).unwrap(); }

    let mut l_shape:    BigUint = BigUint::from_u8(1).unwrap();
    let mut rect:       BigUint = BigUint::from_u8(3).unwrap();

    for _k in (2..n).step_by(2)
    {
        (rect, l_shape) = (&rect * 3u8 + 2u8 * &l_shape, &rect + &l_shape);
    }

    return rect;
}

use rug::{Float};
use rug::ops::*;
// use rug::float::Round;
const PRECISION: u32 = 1 << 20;

fn compute_eigen(n: usize) -> Option<Float>
{
    if n % 2 == 1 || n == 0 {return None;}

    let n = 1 + n / 2;
    let root = Float::with_val(PRECISION, 3).sqrt();
    let l0 = Float::with_val(PRECISION, 2.0 - &root);
    let l1 = Float::with_val(PRECISION, 2.0 + &root);
    let root = -root;

    let one_minus_r = Float::with_val(PRECISION, 1) + &root;
    let three_minus_r = 3 + root;

    let lhs = 3.0 * one_minus_r * l0.pow(n);
    let rhs = three_minus_r * l1.pow(n);
    let result = (lhs + rhs) / 6.0;

    return Some(result);
}

pub fn tiling(n: usize)
{
    let n_ways = compute(n);
    println!("{}", n_ways);
    // println!("{}", n_ways.to_string().len());

    // let r = compute_eigen(n);
    // if r.is_none() {return;}
    // println!("{}", r.unwrap().to_integer_round(Round::Up).unwrap().0);
}