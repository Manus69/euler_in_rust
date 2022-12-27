
use num::Rational32;

fn get_common<T>(lhs: &[T], rhs: &[T]) -> Vec<T>
where T: PartialEq + Copy
{
    let mut common: Vec<T> = Vec::new();

    for k in 0..lhs.len()
    {
        for w in 0..rhs.len()
        {
            if lhs[k] == rhs[w] { common.push(lhs[k]); }
        }
    }

    return common;
}

fn to_int(digits: &[u8]) -> i32
{
    if digits.len() == 0 {return 1;}

    return String::from_utf8_lossy(digits).parse().ok().unwrap();
}

fn cancel_digits(top: i32, bot: i32) -> (i32, i32)
{
    let mut top_digits = top.to_string().as_bytes().to_vec();
    let mut bot_digits = bot.to_string().as_bytes().to_vec();
    let common = get_common(&top_digits, &bot_digits);

    top_digits.retain(|x| !common.contains(x));
    bot_digits.retain(|x| !common.contains(x));
    return (to_int(&top_digits), to_int(&bot_digits));
}

pub fn p33()
{
    let mut product = Rational32::new(1, 1);
    let mut current;
    let (mut _top, mut _bot): (i32, i32);

    for bot in 10..100
    {
        for top in 10..bot
        {
            if top % 10 == 0 || bot % 10 == 0 {continue;}
            current = Rational32::new(top, bot);
            (_top, _bot) = cancel_digits(top, bot);
            if _top == top {continue;}
            if Rational32::new(_top, _bot) == current
            {
                product *= current;
            }
        }
    }

    println!("{}", product.denom());
}