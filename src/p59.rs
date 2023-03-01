use std::{fs};
use std::collections::BTreeMap;
use std::str;

const LETTER: u8 = b' ';
const CIPHER_LEN: usize = 3;

const FILE_NAME: &str = "./src/p059_cipher.txt";

fn read_bytes(file_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>
{
    return Ok(fs::read_to_string(file_name)?.
            split(',').
            map(|c| c.parse::<u8>().unwrap()).
            collect::<Vec<u8>>());
}

fn get_most_frequent(bytes: &Vec<u8>) -> u8
{
    let mut pairs: BTreeMap<u8, usize> = BTreeMap::new();
    let values: Vec<(u8, usize)>;

    for &b in bytes
    {
        pairs.entry(b).and_modify(|e| *e += 1).or_insert(0);
    }

    values = pairs.iter().map(|(k, v)| (*k, *v)).collect();

    return values.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap().0;
}

fn descipher(code: &Vec<u8>, key: &Vec<u8>) -> Vec<u8>
{
    let mut result: Vec<u8> = Vec::new();

    for k in 0..code.len()
    {
        result.push(code[k] ^ key[k % CIPHER_LEN] ^ LETTER);
    }

    return result;
}

pub fn p59()
{
    let bytes = read_bytes(FILE_NAME).ok().unwrap();
    let mut frequent_values = vec![0 as u8; CIPHER_LEN];
    let mut slice: Vec<u8>;
    let result: Vec<u8>;

    for k in 0..CIPHER_LEN
    {
        slice = bytes.
                iter().
                enumerate().
                filter(|&x| x.0 % CIPHER_LEN == k).
                map(|x| *x.1).
                collect(); 

        frequent_values[k] = get_most_frequent(&slice);
    }

    result = descipher(&bytes, &frequent_values);
    println!("{}", str::from_utf8(&result).unwrap());
}