const MAX_DIGITS: usize = 6;

fn digit_power(n: u32, exp: u32) -> u32
{
    return n.to_string().
            as_bytes().
            iter().
            fold(0, | prev, element | 
                            prev + ((*element - b'0') as u32).pow(exp));
}

fn get_numbers(n_digits: usize, exp: u32) -> Vec<u32>
{
    let     limit = 10u32.pow(n_digits as u32);
    let     max = n_digits as u32 * (10u32.pow(exp));
    let mut numbers: Vec<u32> = Vec::new();

    for n in 2..limit
    {
        if n > max { break; }
        if n == digit_power(n, exp)
        { 
            numbers.push(n); 
        }
    }

    return numbers;
}

pub fn p30()
{
    let sum: u32;

    sum = get_numbers(6, 5).iter().sum();

    println!("{}", sum);
}