
use num::{BigUint, FromPrimitive};

const EXP: u32 = 1000;

pub fn p16()
{
    let n = BigUint::from_u32(2).unwrap().pow(EXP);

    println!("{}", n.to_string().as_bytes().iter().map(|b| (b - b'0') as u32).sum::<u32>());
}
