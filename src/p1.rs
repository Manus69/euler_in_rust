
pub fn p1(n: i32) -> i32
{
    let mut sum = 0;

    for k in 1..n
    {
        match (k % 3 == 0) || (k % 5 == 0)
        {
            true => sum += k,
            _ => ()
        }
    }

    return sum;
}