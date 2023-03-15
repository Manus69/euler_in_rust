use std::{ops::{Add, Index, IndexMut}, fmt::format, collections::BTreeSet, iter::Sum, process::Output};
use num::Zero;

const N: usize = 3;

#[derive(Debug, Clone)]
struct NGon<T>
{
    values: Vec<T>,
    current_cell: Option<usize>,
}

impl<'a, T> NGon<T>
where T: Zero + Copy + Ord + ToString
{
    pub fn new(n_sides: usize) -> Self
    {
        return NGon { values: vec![T::zero(); 2 * n_sides], current_cell: Some(0) };
    }

    pub fn init(& mut self, slice: &[T])
    {
        for &value in slice
        {
            self.set_current_cell(value);
        }
    }

    pub fn n_sides(&self) -> usize
    {
        return self.values.len() / 2;
    }

    pub fn is_internal_index(& self, index: usize) -> bool
    {
        return index < self.n_sides();
    }

    pub fn side(& self, n: usize) -> Vec<T>
    {
        return vec![self[self.n_sides() + n], self[n], self[(n + 1) % self.n_sides()]];
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

    fn index(&self, index: usize) -> &Self::Output
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

use crate::euler;
use std::collections;

fn C(n: usize, k: usize) -> usize
{
    let top = euler::factorial(n as u64).unwrap();
    let bot = euler::factorial((n - k) as u64).unwrap();

    return (top / bot) as usize;
}

fn get_index(n: usize, k: usize, perm_index: usize, index_set: & BTreeSet<usize>) -> usize
{
    let n_combinations = C(n, k);
    let perm_index = perm_index % n_combinations;
    let frame_size = n_combinations / n;
    let index = perm_index / frame_size;
    
    return *index_set.range(0..).nth(index).unwrap();
}

fn choose(sequence: & [i32], k: usize, perm_index: usize) -> Vec<i32>
{
    if sequence.len() == 0 || sequence.len() < k {return Vec::new();}

    let mut values: Vec<i32> = Vec::new();
    let mut indicies: BTreeSet<usize> = BTreeSet::from_iter(0..sequence.len());
    let mut n = sequence.len();
    let mut current_index;

    for _k in (1..k + 1).rev()
    {
        current_index = get_index(n, _k, perm_index, &indicies);
        values.push(sequence[current_index]);
        indicies.remove(&current_index);
        n -= 1;
    }
     
     return values;
}

pub fn p68()
{
    let sequence = [1, 2, 3, 4];
    for p in 0..20
    {
        // let c = choose(&sequence, 3, p);
        let c = euler::choose(& sequence, 3, p);
        println!("{:?}", c);
    }

}