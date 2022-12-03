
pub fn p2(n: u64) -> u64
{
    let mut first = 1;
    let mut second = 1;
    let mut next;
    let mut sum = 0;

    while second < n
    {
        next = first + second;

        if next % 2 == 0
        {
            sum += next;
        }

        first = second;
        second = next;
    }

    return sum;
}