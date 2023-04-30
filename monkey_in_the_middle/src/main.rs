use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: (i32, char, i32),
    div_num: i32,
    if_true: i32,
    if_false: i32,
}

impl Monkey {
}

fn parse_operation(data: &str) -> (i32, char, i32) {
    let parse_group = |s: &str, i: usize| -> i32 {
        match s.split(' ').nth(i).unwrap() {
            "old" => -1,
            k => k.parse::<i32>().unwrap(),
        }
    };

    let (mut l, mut op, mut r) = (-1, '\0', -1);

    let mut stmnt: String = data
        .split("= ")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>()
        .join(" ");

    l = parse_group(&stmnt, 0);
    r = parse_group(&stmnt, 2);
    op = stmnt
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<char>()
        .unwrap();

    (l, op, r)
}

fn parse_monkey(data: &str) -> Monkey {
    let mut parts = data.split("\n").skip(1);

    let get_elem = |
    parts: &mut std::iter::Skip<std::str::Split<'_, &'_ str>>,
    split_by: &str
        | {
        parts
            .next()
            .unwrap()
            .split(split_by)
            .nth(1)
            .unwrap()
            .to_string()
    };

    let items: Vec<i32> = get_elem(&mut parts, ": ")
        .split(", ")
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let operation = parse_operation(parts.next().unwrap());

    let div_num: i32 = get_elem(&mut parts, "by ")
        .parse::<i32>()
        .unwrap();

    let (if_true, if_false) = {
        (get_elem(&mut parts, "monkey ").parse::<i32>().unwrap(),
         get_elem(&mut parts, "monkey ").parse::<i32>().unwrap())
    };

    Monkey {
        items,
        operation,
        div_num,
        if_true,
        if_false,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("input.txt")?;
    let data = data.split("\n\n").collect::<Vec<&str>>();

    let mut monkies = Vec::<Monkey>::new();

    for m in data {
        monkies.push(parse_monkey(m));
    }

    Ok(())
}
