const RADIUS: u32 = 1001 / 2;

fn get_spiral(radius: u32) -> (u32, u32)
{
    let (start, end): (u32, u32);

    if radius == 0 { return (1, 1); }

    start = get_spiral(radius - 1).1 + 1;
    end = (2 * radius + 1) * (2 * radius + 1);

    return (start, end);
}

fn compute_sum(start: u32, end: u32) -> u32
{
    let mut current: u32;
    let     side: u32 = (end as f64).sqrt() as u32 - 2;
    let mut sum: u32 = start + side;

    current = sum;
    for _k in 0..3
    {
        current += side + 1;
        sum += current;
    }

    return sum;
}

pub fn p28()
{
    let (mut start, mut end);
    let mut sum = 1;

    for n in 1..=RADIUS
    {
        (start, end) = get_spiral(n);
        sum += compute_sum(start, end);
    }

    println!("{}", sum);
}