fn check_base(base: u64) -> usize
{
    for n in 1..
    {
        if n as f64 > n as f64 * (base as f64).log10() + 1.0 {return n - 1;}
    }

    return 0;
}

pub fn p63()
{
    let mut count = 0;

    for base in 1..10
    {
        count += check_base(base);
    } 

    println!("{}", count);

}