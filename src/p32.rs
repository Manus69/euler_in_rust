use std::collections::HashSet;
use crate::euler;

fn slice_to_number(slice: &[u8]) -> u32
{
    return slice.iter().fold(0, |sum, c| 10 * sum + *c as u32);
}

fn pandigital(lhs: &[u8], rhs: &[u8], result: &[u8]) -> u32
{
    let (_lhs, _rhs, _result): (u32, u32, u32);

    _lhs = slice_to_number(lhs);
    _rhs = slice_to_number(rhs);
    _result = slice_to_number(result);

    return match _lhs * _rhs == _result
    {
        true => _result,
        false => 0
    }
}

fn pandigital_1_4(numbers: &Vec<u8>) -> u32
{
    return pandigital(&numbers[0..1], &numbers[1..5], &numbers[5..]);
}

fn pandigital_2_3(numbers: &Vec<u8>) -> u32
{
    return pandigital(&numbers[0..2], &numbers[2..5], &numbers[5..]);
}

pub fn p32()
{
    let mut set:    HashSet<u32> = HashSet::new();
    let mut digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    while euler::permute(&mut digits)
    {
        set.insert(pandigital_1_4(&digits));
        set.insert(pandigital_2_3(&digits));
    }

    println!("{}", set.iter().sum::<u32>());
}