use std::fs;
use std::error::Error;

fn _get_values(file_name: &str) -> Result<Vec<u32>, Box<dyn Error>>
{
    let     input = fs::read_to_string(file_name)?;
    let mut numbers: Vec<u32> = Vec::new();
    
    for substring in input.split_whitespace()
    {
        numbers.push(substring.parse()?);
    }

    return Ok(numbers);
}

#[derive(Debug)]
struct Grid
{
    numbers: Vec<u32>,
    width: usize,
    height: usize
}

impl Grid
{
    fn new(numbers: Vec<u32>, width: usize, height: usize) -> Option<Grid>
    {
        return match numbers.len() == width * height
        {
            true => Some(Grid {numbers, width, height}),
            false => None
        };
    }

    fn get(&self, row: usize, col: usize) -> Option<&u32>
    {
        return self.numbers.get(row * self.width + col);
    }

    pub fn map_index(&self, index: usize) -> (usize, usize)
    {
        return (index / self.width, index % self.height);
    }

    pub fn product_row(&self, row: usize, col: usize, length: usize) -> Option<u32>
    {
        let mut result = 1;

        for k in 0..length
        {
            result *= self.get(row, col + k).unwrap_or(&0);
        }

        return Some(result);
    }

    pub fn product_col(&self, row: usize, col: usize, length: usize) -> Option<u32>
    {
        let mut result = 1;

        for k in 0..length
        {
            result *= self.get(row + k, col).unwrap_or(&0);
        }

        return Some(result);
    }

    pub fn product_left_right(&self, row: usize, col: usize, length: usize) -> Option<u32>
    {
        let mut result = 1;

        for k in 0..length
        {
            result *= self.get(row + k, col + k).unwrap_or(&0);
        }

        return Some(result);
    }

    pub fn product_right_left(&self, row: usize, col:usize, length: usize) -> Option<u32>
    {
        if col < length { return None; }
        let mut result = 1;

        for k in 0..length
        {
            result *= self.get(row + k, col - k).unwrap_or(&0);
        }

        return Some(result);
    }
}

fn _compute_and_push(grid: &Grid, products: &mut Vec<u32>, index: usize, length: usize,
                    f: Box<dyn Fn(&Grid, usize, usize, usize) -> Option<u32>>)
{
    let (row, col) = grid.map_index(index);
    let value = f(grid, row, col, length);

    if value.is_some() { products.push(value.unwrap()); }
}

fn max(grid: &Grid, length: usize) -> u32
{
    let mut products: Vec<u32> = Vec::new();

    for k in 0..grid.numbers.len()
    {
        _compute_and_push(grid, &mut products, k, length, Box::new(Grid::product_row));
        _compute_and_push(grid, &mut products, k, length, Box::new(Grid::product_col));
        _compute_and_push(grid, &mut products, k, length, Box::new(Grid::product_left_right));
        _compute_and_push(grid, &mut products, k, length, Box::new(Grid::product_right_left));
    }

    return *products.iter().max().unwrap();
}

pub fn p11_test(file_name: &str, length: usize)
{
    let grid = Grid::new(_get_values(file_name).unwrap(), 20, 20);
    let grid = grid.unwrap();
    let max = max(&grid, length);

    println!("{}", max);
}

pub fn p11()
{
    p11_test("./src/p11.txt", 4);
}