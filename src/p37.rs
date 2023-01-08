
//a + b + c = p ; a = p - b - c
//a^2 + b^2 = c^2
// -> (p - b - c)^2 + b^2 = c^2

fn count_solutions(p: usize) -> usize
{
    let mut c = p / 2;
    let mut count: usize = 0;

    while 3 * c > p
    {
        for b in 1..c
        {
            if (p - b - c) * (p - b - c) + b * b == c * c
            {
                count += 1;
            }
        }
        c = c - 1;
    }

    return count;
}

const N: usize = 1000;

pub fn p37()
{
    let (mut max, mut max_p): (usize, usize) = (0, 0);
    let mut current: usize;

    for p in 3..N
    {
        current = count_solutions(p);
        if current > max
        {
            max = current;
            max_p = p;
        }
    }

    println!("{}", max_p);
}