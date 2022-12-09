use num::{FromPrimitive};

const N: u32 = 100;

pub fn p20()
{
    let mut value = num::BigUint::from_u32(1).unwrap();

    for n in 2..=N
    {
        value *= n;
    }

    println!("{}", value.to_string().as_bytes().
                        iter().fold(0 as u32, |a, x| a + *x as u32 - b'0' as u32));
}