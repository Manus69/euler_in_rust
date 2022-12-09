use std::cmp;
use std::fs;

const TEST: &str = "3
7 4
2 4 6
8 5 9 3";

const PATH: &str = "./src/p18.txt";

#[derive(Debug)]
struct Triangle
{
    values: Vec<Vec<u32>>
}

impl Triangle
{
    fn new(string: &str) -> Triangle
    {
        let mut values: Vec<Vec<u32>> = Vec::new();

        for (n, line) in string.lines().enumerate()
        {
            values.push(Vec::new());
            values[n].extend(line.
                        split_whitespace().
                        map(|v| v.parse::<u32>().unwrap()));
        }

        return Triangle {values};
    }

    fn height(&self) -> usize
    {
        return self.values.len();
    }

    fn get(&self, row: usize, col: usize) -> Option<&u32>
    {
        if row >= self.height() { return None; }

        return self.values[row].get(col);
    }

    fn get_row(&self, row: usize) -> &Vec<u32>
    {
        return &self.values[row];
    }
}

#[derive(Debug)]
struct Paths
{
    triangle: Triangle,
    values: Vec<Vec<u32>>
}

impl Paths
{
    fn new(triangle: Triangle) -> Paths
    {
        let mut values: Vec<Vec<u32>> = Vec::new();

        for n in 0..triangle.height()
        {
            values.push(vec![0; n + 1]);
        }

        return Paths {triangle, values};
    }

    fn comute_value(&mut self, row: usize, col: usize) -> u32
    {
        if row + 1 == self.triangle.height()
        {
            return *self.triangle.get(row, col).unwrap();
        }

        return  *self.triangle.get(row, col).unwrap() + 
                cmp::max(self.values[row + 1][col], 
                        self.values[row + 1][col + 1]);
    }

    fn compute_row(&mut self, row: usize)
    {
        let mut value: u32;

        for k in 0..self.triangle.get_row(row).len()
        {
            value = self.comute_value(row, k);
            self.values[row][k] = value;
        }
    }

    fn compute_all(&mut self)
    {
        let row = self.triangle.height() - 1;

        for k in (0..=row).rev()
        {
            self.compute_row(k);
        }
    }
}

pub fn p18()
{
    // let triangle = Triangle::new(TEST);
    let     string = fs::read_to_string(PATH).expect("whatever");
    let mut paths = Paths::new(Triangle::new(string.as_str()));
    paths.compute_all();

    println!("{}", paths.values[0][0]);

}