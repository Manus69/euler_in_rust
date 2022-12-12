
use num::{BigUint, FromPrimitive};

const N_DIGITS: usize = 1000;

pub fn p25()
{
    let (mut f0, mut f1, mut next): (BigUint, BigUint, BigUint);
    let mut result: usize = 0;

    f0 = BigUint::from_u32(0).unwrap();
    f1 = BigUint::from_u32(1).unwrap();

    for k in 0..
    {
        next = &f0 + &f1;

        if next.to_string().len() >= N_DIGITS
        {
            result = k;
            break;
        }

        f0 = f1;
        f1 = next;
    }

    println!("{}", result + 2);
}