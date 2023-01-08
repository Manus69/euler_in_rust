use crate::euler;

fn get_biggest_prime(digits: &[u8]) -> Option<u64>
{
    let mut n: u64;
    let mut result: Option<u64> = None;
    let mut digits: Vec<u8> = digits.to_vec();

    loop
    {
        n = std::str::from_utf8(&digits).ok().unwrap().
            parse().ok().unwrap();
        
        if euler::is_prime(n) {result = Some(n)}
        if euler::permute(&mut digits) == false {break}
    }

    return result;
}

const DIGITS: &str = "123456789";

pub fn p41()
{
    let mut n = DIGITS.len();
    let mut result: Option<u64> = None;

    while n > 3
    {
        result = get_biggest_prime(DIGITS[0..n].as_bytes());
        if result.is_some() {break}
        n -= 1;
    }

    println!("{}", result.unwrap());
}