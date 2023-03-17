use num::Rational64;

fn get_fraction(denom: i64, limit: & Rational64) -> Rational64
{
    let num = limit.numer() * denom / limit.denom();
    let fraction = Rational64::new(num, denom);

    return fraction;
}

const MAX: i64 = 1_000_000;

pub fn p71()
{
    let target = Rational64::new(3, 7);
    
    let max = *(2..MAX).map(|d|get_fraction(d, & target)).
                        filter(|q| q < &target).
                        max().
                        unwrap().
                        numer();

    println!("{}", max);
}