use crate::euler;
use std::collections::{BTreeSet};

const N_DIGITS: u32 = 4;
const SET_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
enum NumberType
{
    Tri = 0, Sq, Pent, Hex, Hept, Oct
}

fn match_number(n: usize) -> NumberType
{
    return match n
    {
        0 => NumberType::Tri,
        1 => NumberType::Sq,
        2 => NumberType::Pent,
        3 => NumberType::Hex,
        4 => NumberType::Hept,
        _ => NumberType::Oct,
    }
}

#[derive(Debug)]
struct Number
{
    value: u64,
    _type: NumberType,
}

impl Number
{
    pub fn new(value: u64, _type: NumberType) -> Self
    {
        return Self {value, _type}
    }
}

fn triangle(n: u64) -> u64
{
    return (n * (n + 1)) / 2;
}

fn square(n: u64) -> u64
{
    return n * n;
}

fn pentagonal(n: u64) -> u64
{
    return (n * (3 * n - 1)) / 2;
}

fn hexagonal(n: u64) -> u64
{
    return n * (2 * n - 1);
}

fn heptagonal(n: u64) -> u64
{
    return (n * (5 * n - 3)) / 2;
}

fn octagonal(n: u64) -> u64
{
    return n * (3 * n - 2);
}

fn get_values(f: &fn(u64) -> u64) -> BTreeSet<u64>
{
    let mut values = BTreeSet::<u64>::new();
    let mut value;
    let mut n_digits;

    for n in 1..
    {
        value = f(n);
        n_digits = euler::n_digits(value);

        if n_digits > N_DIGITS {break;}
        if n_digits == N_DIGITS {values.insert(value);}
    }

    return values;
}

fn get_all_values(functions: &[fn(u64) -> u64]) -> Vec<BTreeSet<u64>>
{
    let mut values: Vec<BTreeSet<u64>> = Vec::new();

    for f in functions
    {
        values.push(get_values(f));
    }

    return values;
}

fn is_cyclic(lhs: u64, rhs: u64) -> bool
{
    return lhs % 100 == rhs / 100;
}

fn map_values(values: &Vec<BTreeSet<u64>>) -> Vec<Number>
{
    let mut numbers: Vec<Number>;

    numbers = values.iter().
            enumerate().
            flat_map(|x| x.1.
                iter().
                map(move |&n| Number::new(n, match_number(x.0)))).
            collect::<Vec<Number>>();

    numbers.sort_by(|a, b| a.value.cmp(&b.value));

    return numbers;
}

fn flag_set(number: &Number, flags: usize) -> bool
{
    return 1 << number._type as usize & flags != 0;
}

fn set_flag(number: &Number, flags: usize) -> usize
{
    return flags | (1 << number._type as usize);
}

fn _chain(numbers: &[Number], current_set: Vec<u64>, set_size: usize, flags: usize) -> Option<Vec<u64>>
{
    let mut result;
    let mut copy;

    if current_set.len() == set_size
    {
        if is_cyclic(current_set[set_size - 1], current_set[0])
        {
            return Some(current_set);
        }

        return None;
    }

    for (k, n) in numbers.iter().enumerate()
    {
        if is_cyclic(*current_set.last().unwrap(), n.value) && !flag_set(n, flags)
        {
            copy = current_set.clone();
            copy.push(n.value);

            result = _chain(&numbers[k..], copy, set_size, set_flag(n, flags));

            if result.is_some() {return result;}
        }
    }

    return None;
}

fn find_set(numbers: &Vec<Number>, set_size: usize) -> Option<Vec<u64>>
{
    let mut result;

    for (k, n) in numbers.iter().enumerate()
    {
        result = _chain(&numbers[k..], vec![n.value], set_size, set_flag(n, 0));

        if result.is_some() {return result;}
    }

    return None;
}

/*
    this shit does not work, but why?
*/
pub fn p61()
{
    let functions = [triangle, 
                                            square, 
                                            pentagonal, 
                                            hexagonal, 
                                            heptagonal, 
                                            octagonal];

    let values = get_all_values(&functions);
    // println!("{:?}", values);

    let numbers = map_values(&values);
    println!("{:?}", numbers);
    println!("{:?}", values[NumberType::Sq as usize]);
    // println!("{:?}", numbers.iter().map(|n| n.value).collect::<Vec<u64>>());

    let result = find_set(&numbers, 6);
    println!("{:?}", result);
    // println!("{}", result.unwrap().iter().sum::<u64>());

}