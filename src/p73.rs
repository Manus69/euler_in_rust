use num::Rational64;

fn to_the_left(fraction: & Rational64, denom: i64) -> Rational64
{
    let num = fraction.numer() * denom / fraction.denom();
    let mut left = Rational64::new(num, denom);

    while *left.denom() != denom
    {
        left -= Rational64::new(1, denom);
    }

    return left;
}

fn to_the_right(fraction: & Rational64, denom: i64) -> Rational64
{
    let numer = fraction.numer() * denom / fraction.denom();
    let mut right = Rational64::new(numer + 1, denom);

    while right <= *fraction
    {
        right += Rational64::new(1, denom);
    }

    while *right.denom() != denom
    {
        right += Rational64::new(1, denom);
    }

    return right;
}

fn count_fractions(lhs: & Rational64, rhs: & Rational64, denom: i64) -> i64
{
    let left = to_the_right(lhs, denom);
    let right = to_the_left(rhs, denom);

    println!("{} {} {}", denom, left, right);
    return match right.numer() - left.numer()
    {
        diff | diff if diff >= 0 => diff + 1,
        _ => 0
    }
}

const MAX: i64 = 100;

//wrong answer
pub fn p73()
{
    let right = Rational64::new(1, 2);
    let left = Rational64::new(1, 3);

    let s = (2..=MAX).
        map(|n| count_fractions(& left, & right, n)).
        sum::<i64>();

    println!("{}", s);
}