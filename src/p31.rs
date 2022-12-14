
fn n_ways(n: i32, coins: &Vec<u8>) -> usize
{
    let     current: u8;
    let     lhs: usize;
    let     rhs: usize;
    let mut leftover: Vec<u8>;

    if n == 0 || n == 1 { return 1; }
    if n < 0 || coins.is_empty() { return 0; }

    leftover = coins.clone();
    current = leftover.pop().unwrap();

    lhs = n_ways(n - current as i32, coins);
    rhs = n_ways(n, &leftover);

    return lhs + rhs;
}

const SUM: i32 = 200;

pub fn p31()
{
    let coins: Vec<u8> = Vec::from([1, 2, 5, 10, 20, 50, 100, 200]);

    println!("{}", n_ways(SUM, &coins));
}