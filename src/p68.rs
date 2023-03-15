use std::{ops::{Index, IndexMut}, collections::BTreeSet};
use num::Zero;
use crate::euler;

const N: usize = 3;

#[derive(Debug, Clone)]
struct NGon<T>
{
    values: Vec<T>,
    used: BTreeSet<T>,
    current_cell: Option<usize>,
}

impl<'a, T> NGon<T>
where T: Zero + Copy + Ord + ToString
{
    pub fn new(n_sides: usize) -> Self
    {
        return NGon { values: vec![T::zero(); 2 * n_sides], used: BTreeSet::new(), current_cell: Some(0) };
    }

    pub fn init(& mut self, slice: &[T])
    {
        for &value in slice
        {
            self.set_current_cell(value);
        }
    }

    pub fn from_slice(n_sides: usize, slice: & [T]) -> NGon<T>
    {
        let mut n_gon: NGon<T> = NGon::new(n_sides);

        n_gon.init(slice);

        return n_gon;
    }

    pub fn n_sides(& self) -> usize
    {
        return self.values.len() / 2;
    }

    pub fn n_cells(& self) -> usize
    {
        return self.values.len();
    }

    pub fn is_internal_index(& self, index: usize) -> bool
    {
        return index < self.n_sides();
    }

    pub fn side(& self, n: usize) -> Vec<T>
    {
        return vec![self[self.n_sides() + n], self[n], self[(n + 1) % self.n_sides()]];
    }

    pub fn current_side_index(& self) -> Option<usize>
    {
        return match self.current_cell
        {
            Some(index) => Some(index % self.n_sides()),
            None => None,
        }
    }

    pub fn next_cell(& mut self)
    {
        self.current_cell = match self.current_cell
        {
            None => Some(0),
            Some(n) =>  match n == 2 * self.n_sides() - 1
                                {
                                    false => Some(n + 1),
                                    true => None,
                                } 
        }
    }

    pub fn set_current_cell(& mut self, value: T)
    {
        let current_cell = self.current_cell.unwrap();

        self[current_cell] = value;
        self.used.insert(value);
        self.next_cell();
    }

    pub fn is_filled(& self) -> bool
    {
        return self.current_cell.is_none();
    }

    fn _find_index_of_least_side(& self) -> usize
    {
        let max = self.values[self.n_sides()..].iter().min().unwrap();

        return self.values.iter().position(|v| *v == *max).unwrap() % self.n_sides();
    }   

    pub fn to_values(& self) -> Vec<T>
    {
        let mut values: Vec<T> = Vec::new();
        // let least_side_index = self._find_index_of_least_side();
        let least_side_index = 0;

        for n in 0..self.n_sides()
        {
            values.extend_from_slice(&self.side((n + least_side_index) % self.n_sides()));
        }

        return values;
    }

    pub fn to_string(& self) -> String
    {
        return self.to_values().
                    iter().
                    map(|v| T::to_string(v)).
                    fold(String::new(), |acc, s| format!("{} {}", acc, s));
    }

}

impl<T> Index<usize> for NGon<T>
{
    type Output = T;

    fn index(&self, index: usize) -> & Self::Output
    {
        return &self.values[index];
    }
}

impl<T> IndexMut<usize> for NGon<T>
{
    fn index_mut(& mut self, index: usize) -> & mut T
    {
        return & mut self.values[index];
    }
}

fn compute_value(n_gon: & NGon<i32>, side_index: usize, target: i32) -> i32
{
    return target - n_gon.side(side_index).iter().sum::<i32>();
}

fn solve_external(mut n_gon: NGon<i32>, target: i32) -> Option<NGon<i32>>
{
    let side_index;
    let value;

    if n_gon.is_filled() {return Some(n_gon);}

    side_index = n_gon.current_side_index().unwrap();
    value = compute_value(&n_gon, side_index, target);

    if !n_gon.used.contains(&value) && value > 0 && value <= n_gon.n_cells() as i32
    {
        n_gon.set_current_cell(value);
        n_gon.used.insert(value);

        return solve_external(n_gon, target);
    }
    
    return None;
}

fn get_initial_sequences(n_sides: usize) -> Vec<Vec<i32>>
{
    let values = (1..=(2 * n_sides as i32)).collect::<Vec<i32>>();
    let set_size = euler::C(2 * n_sides, n_sides);

    return (0..set_size).
            map(|p| euler::choose(&values, n_sides, p)).
            collect::<Vec<Vec<i32>>>();
}

fn get_solutions(initial_sequences: & [Vec<i32>], n_sides: usize, target: i32) -> Vec<NGon<i32>>
{
    let mut solutions: Vec<NGon<i32>> = Vec::new();
    let mut current: Option<NGon<i32>>;

    for sequence in initial_sequences
    {
        current = Some(NGon::from_slice(n_sides, &sequence));
        current = solve_external(current.unwrap(), target);

        if current.is_some() {solutions.push(current.unwrap());}
    }

    return solutions;
}

fn get_target_limits(n_sides: usize) -> (i32, i32)
{
    let max_value = 2 * n_sides as i32;

    return (max_value + 3, 3 * max_value - 3);
}

fn get_all_solutions(n_sides: usize) -> Vec<NGon<i32>>
{
    let mut all_solutions: Vec<NGon<i32>> = Vec::new();
    let (target_min, target_max) = get_target_limits(n_sides);
    let initial_sequences = get_initial_sequences(n_sides);

    initial_sequences.iter().for_each(|s| println!("{:?}", s));
    let mut solutions: Vec<NGon<i32>>;

    for target in target_min..target_max
    {
        solutions = get_solutions(&initial_sequences, n_sides, target);
        all_solutions.append(& mut solutions);
    }

    return all_solutions;
}
 
pub fn p68()
{
    let solutions = get_all_solutions(3);

    let mut solution_strings = solutions.iter().map(|ng| ng.to_string()).collect::<Vec<String>>();
    // solution_strings.sort();
    // println!("{:?}", solution_strings);

    // solution_strings.iter().for_each(|s| println!("{}", s));
}