use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: (i32, char, i32),
    div_num: i32,
    if_true: usize,
    if_false: usize,
    inspections: i32,
}

fn parse_operation(data: &str) -> (i32, char, i32) {
    let parse_group = |s: &str, i: usize| -> i32 {
        match s.split(' ').nth(i).unwrap() {
            "old" => -1,
            k => k.parse::<i32>().unwrap(),
        }
    };

    let stmnt: String = data
        .split("= ")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>()
        .join(" ");

    let l = parse_group(&stmnt, 0);
    let r = parse_group(&stmnt, 2);
    let op = stmnt.split(' ').nth(1).unwrap().parse::<char>().unwrap();

    (l, op, r)
}

fn parse_monkey(data: &str) -> Monkey {
    let mut parts = data.split("\n").skip(1);

    let get_elem = |parts: &mut std::iter::Skip<std::str::Split<'_, &'_ str>>, split_by: &str| {
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

    let operation: (i32, char, i32) = parse_operation(parts.next().unwrap());

    let div_num: i32 = get_elem(&mut parts, "by ").parse::<i32>().unwrap();

    let (if_true, if_false): (usize, usize) = {
        (
            get_elem(&mut parts, "monkey ").parse::<usize>().unwrap(),
            get_elem(&mut parts, "monkey ").parse::<usize>().unwrap(),
        )
    };

    Monkey {
        items,
        operation,
        div_num,
        if_true,
        if_false,
        inspections: 0,
    }
}

fn inspect_items(monkey: &mut Monkey) -> Vec<(usize, i32)> {
    // Monkey, Worry.
    let mut res: Vec<(usize, i32)> = Vec::new();

    for i in 0..monkey.items.len() {
        let worry = match monkey.operation {
            (-1, '*', -1) => monkey.items[i].pow(2),
            (-1, '+', -1) => monkey.items[i] * 2,
            (-1, '*', k) => monkey.items[i] * k,
            (-1, '+', k) => monkey.items[i] + k,
            (k, '*', -1) => k * monkey.items[i],
            (k, '+', -1) => k + monkey.items[i],
            _ => panic!("unreachable"),
        } / 3;
        if worry % monkey.div_num == 0 {
            res.push((monkey.if_true, worry));
        } else {
            res.push((monkey.if_false, worry));
        }
        monkey.inspections += 1;
    }

    monkey.items.clear();
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("input.txt")?;
    let data = data.split("\n\n").collect::<Vec<&str>>();

    let mut monkeys = Vec::<Monkey>::new();

    for m in data {
        monkeys.push(parse_monkey(m));
    }

    for _ in 0..1000 {
        for i in 0..monkeys.len() {
            let items: Vec<(usize, i32)> = inspect_items(&mut monkeys[i]);
            for j in 0..items.len() {
                let throw_to = items[j].0;
                let value = items[j].1;
                monkeys[throw_to].items.push(value);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspections);
    monkeys.reverse();
    println!(
        "{monkey_business}",
        monkey_business = { monkeys[0].inspections * monkeys[1].inspections }
    );

    Ok(())
}
