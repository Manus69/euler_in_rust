
const MAX: usize = 1 << 20;
const N: usize = 101;
const MAX_VALUE: usize = 1_000_000;

fn ceil_add(lhs: usize, rhs: usize, cap: usize) -> usize
{
    return match lhs + rhs < cap
    {
        true => lhs + rhs,
        _ => cap
    }
}

fn pascal_trunc(n_rows: usize) -> Vec<Vec<usize>>
{
    let mut table : Vec<Vec<usize>>= vec![vec![0; n_rows]; n_rows];

    for row in 0..n_rows
    {
        table[row][0] = 1;
        table[row][row] = 1;

        for col in 1..row
        {
            table[row][col] = ceil_add(table[row - 1][col - 1],
                        table[row - 1][col], MAX);
        }
    }

    return table;
}

pub fn p53()
{
    let table = pascal_trunc(N);

    println!("{:?}", table.iter().flatten().filter(|v| *v > &MAX_VALUE).count());
}