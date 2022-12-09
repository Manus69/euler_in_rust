
const N: usize = 21;

#[derive(Debug)]
struct Grid
{
    width:   usize,
    height:  usize,
    values:  Vec<Vec<u64>>
}

impl Grid
{
    fn new(width: usize, height: usize) -> Grid
    {
        let mut values: Vec<Vec<u64>> = Vec::new();
         
        for k in 0..height
        {
            values.push(Vec::new());
            for w in 0..width
            {
                values[k].push(match k + 1 == height || w + 1 == width
                {
                    true => 1,
                    false => 0
                });
            }
        }

        return Grid {width, height, values};
    }

    fn get(&self, row: usize, col: usize) -> u64
    {
        return self.values[row][col];
    }

    fn set(&mut self, row: usize, col: usize, value: u64)
    {
        self.values[row][col] = value;
    }

    fn compute_value(&self, row: usize, col: usize) -> u64
    {
        return self.get(row + 1, col) + self.get(row, col + 1);
    }

    fn fill_row(&mut self, row: usize)
    {
        let     col = self.width - 1;
        let mut value;

        for k in (0..col).rev()
        {
            value = self.compute_value(row, k);
            self.set(row, k, value);
        }
    }

    fn fill(&mut self)
    {
        let row = self.height - 1;

        for k in (0..row).rev()
        {
            self.fill_row(k);
        }
    }

}

pub fn p15()
{
    let mut grid: Grid = Grid::new(N, N);
    grid.fill();

    println!("{:?}", grid.get(0, 0));
}