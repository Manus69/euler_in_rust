
fn is_palindrome(n: u64) -> bool
{
    let n_string: String = n.to_string();
    let reverse: String = n_string.chars().rev().collect();

    return n_string == reverse;
}

pub fn p4(n: u32) -> u64
{
    let mut max: u64 = 0;
    let mut result: u64;
    let limit: u64 = 10u64.pow(n);

    for k in 2..limit
    {
        for w in k..limit
        {
            result = k * w;
            
            if is_palindrome(result) && result > max
            {
                max = result;
            }
        }
    }

    return max;
}