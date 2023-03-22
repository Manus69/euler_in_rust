use std::{fs::{self, read_to_string}, collections::{HashMap, BTreeSet}};

const PATH: & str = "./src/p079_keylog.txt";

fn add_log(log: & Vec<u8>, map: & mut HashMap<u8, BTreeSet<u8>>)
{
    for k in 0..(log.len() - 1)
    {
        map.entry(log[k]).and_modify(|e| {e.insert(log[k + 1]);}).
            or_insert(BTreeSet::from([log[k + 1]]));
    }
}

fn get_after_map(logs: & Vec<Vec<u8>>) -> HashMap<u8, BTreeSet<u8>>
{
    let mut map: HashMap<u8, BTreeSet<u8>> = HashMap::new();

    for log in logs
    {
        add_log(log, &mut map);
    }

    return map;
}

fn find_last(map: & HashMap<u8, BTreeSet<u8>>) -> Option<u8>
{
    return map.
            iter().
            find_map(|(& _number, set)| if set.len() == 1 {set.first().copied()} else {None});
}

fn pop_last(map: & mut HashMap<u8, BTreeSet<u8>>) -> Option<u8>
{
    let last = find_last(map);

    if last.is_none() {return None;}

    map.retain(|_k, v| {v.remove(& last.unwrap()); v.len() != 0});

    return last;
}

fn form_sequence(map: & mut HashMap<u8, BTreeSet<u8>>) -> Vec<u8>
{
    let mut sequence: Vec<u8> = Vec::new();

    loop
    {
        match map.len()
        {
            1 =>
            {
                sequence.extend_from_slice(& [* map.iter().nth(0).unwrap().1.first().unwrap(),
                                                 * map.iter().nth(0).unwrap().0]);
                break;
            }
            _ => match pop_last(map)
            {
                None => break,
                Some(v) => sequence.push(v),
            }
        }
    }

    return sequence;
}

pub fn p79()
{
    let logs = read_to_string(PATH).
                    unwrap().
                    split("\n").
                    map(|s| s.as_bytes().iter().map(|c| *c - b'0').collect::<Vec<u8>>()).
                    collect::<Vec<Vec<u8>>>();

    let mut map = get_after_map(& logs);
    
    let sequence = form_sequence(& mut map);
    // println!("{:?}", sequence);
    sequence.into_iter().rev().map(|v| print!("{}", v)).count();
}