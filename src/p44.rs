
//Pk = k(3k - 1) / 2
fn is_pentagon(n: usize) -> bool
{
    return ((1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0).fract() == 0.0;
}

fn pentagon(n: usize) -> usize
{
    return (n * (3 * n - 1)) / 2;
}


pub fn p44()
{
    let mut diff_current: usize;
    let mut pk: usize;
    let mut pj: usize;

    for k in 2..
    {
        pk = pentagon(k);

        for j in 1..k
        {
            pj = pentagon(j);
            diff_current = pk - pj;

            if is_pentagon(diff_current) && is_pentagon(pk + pj)
            {
                println!("{}", diff_current);
                return;
            }
        }
    }
}