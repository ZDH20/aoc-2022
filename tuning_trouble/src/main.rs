use std::fs;
use std::collections::HashSet;

fn part1(data: &String) -> Option<u32> {
    for i in 0..data.len() - 4 {
        let set: HashSet<char> = data[i..i + 4].chars().collect();
        if set.len() == 4 { return Some(i as u32 + 4); }
    }
    None
}

fn part2(data: &String) -> Option<u32> {
    for i in 0..data.len() - 14 {
        let set: HashSet<char> = data[i..i + 14].chars().collect();
        if set.len() == 14 { return Some(i as u32 + 14); }
    }
    None
}

fn main() -> Result<(), std::io::Error> {
    let data = fs::read_to_string("input.txt")?;

    println!("{}", part2(&data).unwrap());

    Ok(())
}
