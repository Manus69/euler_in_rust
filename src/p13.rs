use num::{self, BigUint};
use std::{fs, str::FromStr};
use std::error::Error;

const N_DIGITS: u32 = 10;
const FILE_NAME: &str = "./src/p13.txt";

fn _parse_file(file_name: &str) -> Result<Vec<num::BigUint>, Box<dyn Error>>
{
    let     data = fs::read_to_string(file_name)?;
    let mut vector: Vec<num::BigUint> = Vec::new();

    for _str in data.lines()
    {
        // let x = BigUint::from_str(_str)?;
        // println!("{}", x);
        vector.push(BigUint::from_str(_str)?);
    }

    return Ok(vector);
}

fn p13_test(file_name: &str) -> Result<num::BigUint, Box<dyn Error>>
{
    let numbers = _parse_file(file_name)?;

    return Ok(numbers.iter().sum());
}

pub fn p13()
{
    match p13_test(FILE_NAME)
    {
        Ok(number) => println!("{}", &number.to_string()[0..N_DIGITS as usize]),
        Err(_) => ()
    }
}