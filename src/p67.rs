use std::{fs, collections::HashMap};
use core::str;

const FILE_NAME: &str = "./src/p067_triangle.txt";
const TEST: &str = "3\n7 4\n2 4 6\n8 5 9 3\n";


#[derive(Debug)]
struct Triangle
{
    numbers: Vec<Vec<u32>>
}

impl Triangle
{
    pub fn at<'a>(&'a self, row: usize, col: usize) -> Option<&'a u32>
    {
        return match self.numbers.get(row)
        {
            None => None,
            Some(_row) => _row.get(col)
        }
    }

    pub fn new(text: String) -> Result<Triangle, Box<dyn std::error::Error>>
    {
        let mut numbers: Vec<Vec<u32>> = Vec::new();

        for (index, row) in text.split("\n").enumerate()
        {
            if row.len() == 0 {break;}

            numbers.push(Vec::new());
            for col in row.split_ascii_whitespace()
            {
                numbers[index].push(str::parse(col)?);
            }
        }

        return Ok(Triangle { numbers: numbers });
    }

}

fn _find(triangle: &Triangle, row: usize, col: usize, table: &mut HashMap<(usize, usize), u32>) -> u32
{
    let value;
    let lhs;
    let rhs;
    let max;

    if row == triangle.numbers.len() - 1 {return *triangle.at(row, col).unwrap();}
    value = triangle.at(row, col);
    if value.is_none() {return 0;} 
    if table.contains_key(&(row, col)) {return table[&(row, col)];}

    lhs = _find(triangle, row + 1, col, table);
    rhs = _find(triangle, row + 1, col + 1, table);
    max = u32::max(lhs, rhs);

    table.insert((row, col), value.unwrap() + max);

    return value.unwrap() + max;
}

fn find_path_sum(triangle: &Triangle) -> u32
{
    let mut table: HashMap<(usize, usize), u32> = HashMap::new();

    return _find(triangle, 0, 0, &mut table);
}

pub fn p67()
{
    let text = fs::read_to_string(FILE_NAME).unwrap();
    let triangle = Triangle::new(text);
    // println!("{:?}", triangle);

    let path = find_path_sum(&triangle.unwrap());
    println!("{}", path);

}