use std::fmt::Binary;


fn is_palindrome<T>(array: &[T]) -> bool
where T: PartialEq
{
    if array.len() < 2 {return true;}

    if array[0] == *array.last().unwrap()
    {
        return is_palindrome(&array[1..array.len() - 1]);
    }

    return false;
}

fn is_bd_palindrome(n: u64) -> bool
{
    let bin = format!("{:b}", n);
    let dec = n.to_string();

    return is_palindrome(bin.as_bytes()) && is_palindrome(dec.as_bytes());
}

const MAX: u64 = 1_000_000;

pub fn p36()
{
    let mut sum: u64 = 0;

    for k in 0..MAX
    {
        if is_bd_palindrome(k) {sum += k}
    }

    println!("{}", sum);
}