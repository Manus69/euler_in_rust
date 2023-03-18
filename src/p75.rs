use std::collections::HashMap;

use crate::euler;

fn get_triple(m: usize, n: usize) -> (usize, usize, usize)
{
    return (m * m - n * n, 2 * m * n, m * m + n * n);
}

fn _check_condition(m: usize, n: usize) -> bool
{
    if ((m % 2) ^ (n % 2)) == 0 {return false;}
    if num::integer::gcd(m, n) != 1 {return false;}

    return true;
}

fn get_primitive_triples(perimeter_limit: usize) -> Vec<(usize, usize, usize)>
{
    let mut triples: Vec<(usize, usize, usize)> = Vec::new();
    let last_m = (perimeter_limit as f32).sqrt() as usize + 1;

    for m in 1..last_m
    {
        for n in 1..m
        {
            if _check_condition(m, n)
            {
                triples.push(get_triple(m, n));
            }
        }
    }

    return triples;
}

fn add_triple(triple: & (usize, usize, usize), table: & mut HashMap<usize, usize>, p_limit: usize)
{
    let p = triple.0 + triple.1 + triple.2;
    let mut factor = 1;

    while p * factor <= p_limit
    {
        table.entry(p * factor).and_modify(|v| *v += 1).or_insert(1);
        factor += 1;
    }
}

fn count_triples(primitive_triples: & Vec<(usize, usize, usize)>, p_limit: usize) -> HashMap<usize, usize>
{
    let mut table: HashMap<usize, usize> = HashMap::new();

    for triple in primitive_triples
    {
        add_triple(triple, &mut table, p_limit);
    }

    return table;
}

const MAX: usize = 1_500_000;

pub fn p75()
{
    let triples = get_primitive_triples(MAX);
    let counts = count_triples(& triples, MAX);
    // println!("{:?}", counts);

    let x = counts.iter().filter(|(& _p, & count)| count == 1).count();
    println!("{}", x);
}