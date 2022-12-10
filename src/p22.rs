use std::fs;

const PATH: &str = "./src/p022_names.txt";

fn score(name: &str) -> usize
{
    let mut _score: usize = 0;

    for c in name.as_bytes()
    {
        if *c != b'\"' { _score += (c - b'A' + 1) as usize; }
    }

    return _score;
}

fn compute_scores(names: &Vec<&str>) -> usize
{
    return names.iter().
                enumerate().
                map(|pair| (pair.0 + 1) * score(*pair.1)).
                sum();
}

pub fn p22()
{
    let     content = fs::read_to_string(PATH).expect("ass");
    let mut names: Vec<&str> = content.split(',').collect();

    names.sort();

    println!("{}", compute_scores(&names));
}