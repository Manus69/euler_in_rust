
fn _sum_n(n: u64) -> u64
{
    return (n * (n + 1)) / 2;
}

fn _sum_squares(n: u64) -> u64
{
    return (n * (n + 1) * (2 * n + 1)) / 6;
}

pub fn p6(n: u64) -> u64
{
    let mut sum = _sum_n(n);

    sum = sum * sum;

    return sum - _sum_squares(n);
}