
//a^2 + b^2 = c^2
//a + b + c = k ; a = k - b - c; a = k - (b + c)

const N: u32 = 1000;

fn find_product() -> u32
{
    let mut diff: u32;

    for b in 1..N
    {
        for c in b..N
        {
            if b + c >= N { break; }

            diff = N - b - c;
            if diff * diff + b * b == c * c { return diff * b * c; }
        }
    }

    return 0;
}

pub fn p9()
{
    println!("{}", find_product());
}