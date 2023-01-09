
use crate::euler;

fn check_slice_divisibility(slice: &[u8], divisor: i32) -> bool
{
    let product: i32 = std::str::from_utf8(slice)
                    .ok()
                    .unwrap()
                    .parse()
                    .ok()
                    .unwrap();

    return product % divisor == 0;
}

const DIVISORS: [i32; 7] = [2, 3, 5, 7, 11, 13, 17];

pub fn p43()
{
    let mut digits = "0123456789".as_bytes().to_vec();
    let mut sum: usize = 0;
    let mut index: usize;

    while euler::permute(&mut digits)
    {
        index = 0;
        if digits[0] == b'0' {continue}

        for slice in digits[1..].windows(3)
        {
            if check_slice_divisibility(slice, DIVISORS[index]) == false
            {
                break;
            }

            index += 1;
        }

        if index == DIVISORS.len()
        {
            sum += std::str::from_utf8(&digits)
                .ok()
                .unwrap()
                .parse::<usize>()
                .ok()
                .unwrap();
        }
    }

    println!("{}", sum);
}