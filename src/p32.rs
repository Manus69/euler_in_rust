
use crate::euler;

fn slice_to_number(slice: &[u8]) -> u32
{
    return slice.iter().fold(0, |sum, c| 10 * sum + *c as u32);
}

fn pandigital(lhs: &[u8], rhs: &[u8], result: &[u8]) -> Option<u32>
{
    let (_lhs, _rhs, _result): (u32, u32, u32);

    _lhs = slice_to_number(lhs);
    _rhs = slice_to_number(rhs);
    _result = slice_to_number(result);

    return match _lhs * _rhs == _result
    {
        true => Some(_result),
        false => None
    }
}

fn pandigital_1_4(numbers: &Vec<u8>) -> u32
{
    return match pandigital(&numbers[0..1], &numbers[1..5], &numbers[5..])
    {
        Some(n) => n,
        None => 0
    }
}

fn pandigital_2_3(numbers: &Vec<u8>) -> u32
{
    return match pandigital(&numbers[0..2], &numbers[2..5], &numbers[5..])
    {
        Some(n) => n,
        None => 0
    }
}

pub fn p32()
{
    let mut digits: Vec<u8> = vec![3, 9, 1, 8, 6, 7, 2, 5, 4];
    
    while euler::permute(&mut digits)
    {
        println!("{:?}", digits);
    }

}