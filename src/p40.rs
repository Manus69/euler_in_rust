const N_DIGITS: usize = 1_000_000;

fn compute(digits: &str) -> usize
{
    let mut n = 1;
    let mut product = 1;

    while n <= N_DIGITS
    {
        product *= (digits.as_bytes()[n] as u8 - b'0') as usize;
        n *= 10;
    }

    return product;
}

pub fn p40()
{
    let mut n = 1;
    let mut digits: String = String::from("0");

    while digits.len() <= N_DIGITS
    {
        digits += &n.to_string();
        n += 1;
    }

    println!("{}", compute(&digits));
}