const N: u64 = 1_000_001;

fn fill(divisor_map: & mut [u64], index: usize, value: u64)
{
    let mut _index = index << 1;

    while _index < divisor_map.len()
    {
        divisor_map[_index] = value;
        _index += index;
    }
}

fn sieve_map(n: u64) -> Vec<u64>
{
    let mut divisor_map: Vec<u64> = vec![0; n as usize];

    for index in 2..n as usize
    {
        if divisor_map[index] == 0
        {
            fill(& mut divisor_map, index, index as u64);
        }
    }

    divisor_map[1] = 1;

    return divisor_map;
}

fn phi(n: u64, divisor_map: & Vec<u64>, phi_table: & mut Vec<u64>) -> u64
{
    let divisor = divisor_map[n as usize];
    let remainder;
    let result;
    let gcd;
    let phi_gcd;

    if divisor == 0 {return n - 1;}
    if phi_table[n as usize] != 0 {return phi_table[n as usize];}

    remainder = n / divisor;
    gcd = num::integer::gcd(remainder, divisor);
    phi_gcd = phi(gcd, divisor_map, phi_table);

    result = (phi(divisor, divisor_map, phi_table) * 
            phi(remainder, divisor_map, phi_table) *
            gcd) / phi_gcd;

    phi_table[n as usize] = result;

    return result;
}

pub fn p69()
{
    let map = sieve_map(N);
    let mut phi_map = vec![0; N as usize];
    let mut _phi;
    let mut max = 0f64;
    let mut max_index = 0;
    let mut current;

    phi_map[1] = 1;
    for n in 2..N
    {
        _phi = phi(n, &map, & mut phi_map);
        // println!("{} {}", n, _phi);
        current = n as f64 / _phi as f64;
        if current > max
        {
            max = current;
            max_index = n;
        }
    }
    println!("{}", max_index);
}