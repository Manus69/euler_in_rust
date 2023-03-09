use std::collections::HashMap;

use crate::euler;

fn encode_digits(n: u64) -> u64
{
    let mut encoding = 0u64;
    let mut n = n;
    let mut digit;

    while n > 0
    {
        digit = n % 10;
        encoding += 10u64.pow(1 + digit as u32);

        n /= 10;
    }
    
    return encoding;
}

fn find_permutation(low: u64, length: usize) -> u64
{
    let mut table: HashMap<u64, (usize, u64)> = HashMap::new();
    let mut entry;
    let mut cube;
    let mut digit_code;
    let mut candidate = 0u64;

    for n in low..
    {
        cube = n * n * n;
        digit_code = encode_digits(cube);
        entry = table.entry(digit_code).
                    and_modify(|e| e.0 += 1).
                    or_insert((1, cube));

        if entry.0 == length
        {
            if candidate == 0
            {
                candidate = entry.1;
                continue;
            }

            if euler::n_digits(candidate) < euler::n_digits(cube) {return candidate;}
        }

    }

    return 0;
}

pub fn p62()
{
    let x = find_permutation(1, 5);
    println!("{}", x);

}