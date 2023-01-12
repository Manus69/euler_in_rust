use std::collections::HashSet;
use std::collections::HashMap;

fn to_digits(n: usize, table: &mut HashMap<usize, HashSet<u8>>) -> &HashSet<u8>
{
    if table.contains_key(&n) {return &table[&n]}
    let digits;

    digits = n.to_string().into_bytes().iter().map(|c| *c).collect();
    table.insert(n, digits);

    return &table[&n];
}

fn check_number(n: usize, table: &mut HashMap<usize, HashSet<u8>>) -> bool
{
    let digits = to_digits(n, table).clone();
    let mut next = n + n;
    
    for _k in 0..5
    {
        if *to_digits(next, table) != digits
        {
            return false;
        }
        next += n;
    }

    return true;
}

pub fn p52()
{
    let mut table: HashMap<usize, HashSet<u8>> = HashMap::new();

    for n in 1..
    {
        if check_number(n, &mut table)
        {
            println!("{}", n);
            break;
        }
    }
}