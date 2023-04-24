use std::fs;

type StackVec = Vec<(char, usize)>;
type InstrVec = Vec<(usize, usize, usize)>;

fn parse_input(input: &String) -> (StackVec, InstrVec) {
    let (stacks_info, instrs_info) = {
        let mut iter = input.split("\n\n");
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut letters = StackVec::new();
    let mut instrs = InstrVec::new();

    stacks_info.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, letter)| {
            if letter.is_alphabetic() {
                letters.push((letter, (i - 1) / 4));
            }
        });
    });

    instrs_info.lines().for_each(|line| {
        let (mut group, mut idx) = ([0; 3], 0);
        line.split(' ').for_each(|word| {
            if let Ok(num) = word.parse::<usize>() {
                group[idx] = num;
                idx += 1;
                if idx == 3 {
                    return;
                }
            }
        });
        instrs.push((group[0], group[1], group[2]));
    });

    (letters, instrs)
}

struct StackList {
    stacks: Vec<Vec<char>>,
}

impl StackList {
    fn new(letters: &StackVec, sz: usize) -> Self {
        let mut stacks: Vec<Vec<char>> = vec![vec![]; sz];
        letters.iter().rev().for_each(|(letter, idx)| {
            stacks[*idx].push(*letter);
        });
        Self { stacks }
    }

    fn move_elems(&mut self, count: usize, src: usize, dest: usize) {
        (0..count).for_each(|_| {
            let x: char = self.stacks[src - 1].pop().unwrap();
            self.stacks[dest - 1].push(x);
        });
    }

    fn move_elems_part2(&mut self, count: usize, src: usize, dest: usize) {
        let mut tmp = Vec::<char>::new();
        (0..count).for_each(|_| {
            let x: char = self.stacks[src - 1].pop().unwrap();
            tmp.push(x);
        });

        tmp.iter().rev().for_each(|x| {
            self.stacks[dest - 1].push(*x);
        })
    }

    fn print_last_elems(&self) {
        self.stacks.iter().for_each(|stack| {
            if let Some(n) = stack.last() {
                print!("{n}");
            }
        });
    }

    fn dump(&self) {
        self.stacks.iter().enumerate().for_each(|(i, stack)| {
            println!("{i}: {stack:?}");
        });
    }
}

fn main() -> Result<(), std::io::Error> {
    let (letters_and_idx, instrs) = parse_input(&fs::read_to_string("input.txt")?);

    let mut stacks = StackList::new(&letters_and_idx, 10);

    instrs.iter().for_each(|instr| {
        stacks.move_elems_part2(instr.0, instr.1, instr.2);
    });

    stacks.print_last_elems();

    Ok(())
}
