
fn is_triangular(n: usize) -> bool
{
    return (0.5 + (1.0 + 8.0 * n as f64).sqrt() / 2.0).fract() == 0.0;
}

fn is_pentagonal(n: usize) -> bool
{
    return ((1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0).fract() == 0.0;
}

fn is_hexagonal(n: usize) -> bool
{
    return (0.25 + (1.0 + 8.0 * n as f64).sqrt() / 4.0).fract() == 0.0;
}

const START: usize = 40756;
const START_INDEX: usize = 144;

pub fn p45()
{
    let mut value: usize = 0;

    for n in START_INDEX..
    {
        value = n * (2 * n - 1);
        if is_triangular(value) && is_pentagonal(value)
        {
            break;
        }
    }

    println!("{}", value);
}