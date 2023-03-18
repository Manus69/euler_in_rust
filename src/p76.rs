use std::collections::{BTreeSet, HashMap};

fn n_ways(n: usize, numbers: BTreeSet<usize>, table: & mut HashMap<(usize, BTreeSet<usize>), usize>) -> usize
{
    let mut new_set: BTreeSet<usize>;
    let current;
    let n_ways_using;
    let n_ways_not_using;

    if n == 0 {return 1;}
    if numbers.len() == 0 {return 0;}

    if table.contains_key(& (n, numbers.clone())) {return table[& (n, numbers)];}

    new_set = numbers.clone();
    current = *new_set.first().unwrap();
    new_set.remove(& current);

    n_ways_using = match current
    {
        c if c <= n => n_ways(n - current, numbers.clone(), table),
        _ => 0,
    };

    n_ways_not_using = n_ways(n, new_set, table);

    table.insert((n, numbers), n_ways_using + n_ways_not_using);

    return n_ways_using + n_ways_not_using;
}

fn n_ways_to_represent(n: usize) -> usize
{
    let numbers = (1..n).collect::<BTreeSet<usize>>();
    let mut table: HashMap<(usize, BTreeSet<usize>), usize> = HashMap::new();

    return n_ways(n, numbers, & mut table);
}

//this code is shit
pub fn p76()
{
    let x = n_ways_to_represent(100);

    println!("{}", x);
}