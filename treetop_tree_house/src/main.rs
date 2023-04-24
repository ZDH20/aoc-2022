use std::fs;

const PATH: &str = "input.txt";
const ROWS: usize = 99;
const COLS: usize = 99;

fn input_to_vec(contents: &String) -> [[u32; COLS]; ROWS] {
    let data: Vec<u32> = contents
        .chars()
        .filter(|c| *c != '\n')
        .map(|c: char| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut grid: [[u32; COLS]; ROWS] = [[0; COLS]; ROWS];

    for i in 0..ROWS {
        for j in 0..COLS {
            grid[i][j] = data[i * COLS + j];
        }
    }

    grid
}

fn init_visible() -> [[bool; COLS]; ROWS] {
    let mut mat = [[false; COLS]; ROWS];

    for i in 0..ROWS {
        (mat[i][0], mat[i][COLS - 1]) = (true, true);
        mat[i][0] = true;
    }

    for i in 0..COLS {
        (mat[0][i], mat[ROWS - 1][i]) = (true, true);
    }

    mat
}

fn scan(
    grid: &[[u32; COLS]; ROWS],
    visible: &mut [[bool; COLS]; ROWS],
    (i, j): (usize, usize)) {

    let t = grid[i][j];

    // Scan up
    if (0..i).rev().all(|row| grid[row][j] < t) {
        visible[i][j] = true;
    }

    // Scan down
    if (i + 1..ROWS).all(|row| grid[row][j] < t) {
        visible[i][j] = true;
    }

    // Scan left
    if (0..j).rev().all(|col| grid[i][col] < t) {
        visible[i][j] = true;
    }

    // Scan right
    if (j + 1..COLS).all(|col| grid[i][col] < t) {
        visible[i][j] = true;
    }
}

fn scan2(
    grid: &[[u32; COLS]; ROWS],
    (i, j): (usize, usize)) -> u32 {

    let (pos, mut res, mut count) = (grid[i][j], 1, 0);

    // Scan up
    for row in (0..i).rev() {
        let elem = grid[row][j];
        if pos > elem {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    println!("up: {count}");
    res *= count;
    count = 0;

    // Scan down
    for row in i + 1..ROWS {
        let elem = grid[row][j];
        if pos > elem {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }

    println!("down: {count}");
    res *= count;
    count = 0;

    // Scan left
    for col in (0..j).rev() {
        let elem = grid[i][col];
        if pos > elem {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }

    println!("left: {count}");
    res *= count;
    count = 0;

    // Scan right
    for col in j + 1..COLS {
        let elem = grid[i][col];
        if pos > elem {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    println!("right: {count}");

    res *= count;

    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = input_to_vec(&fs::read_to_string(PATH)?);
    let mut visible = init_visible();

    // Part 1
    for i in 0..ROWS {
        for j in 0..COLS {
            scan(&grid, &mut visible, (i, j));
            print!("{bool_as_u8}", bool_as_u8 = visible[i][j] as u8);
        }
        println!();
    }

    println!("{visible_trees}", visible_trees = {
        visible
            .iter()
            .flatten()
            .filter(|x| **x)
            .count()
    });

    println!("-------");

    // Part 2
    let mut max = 0;
    for i in 0..ROWS {
        for j in 0..COLS {
            println!("Scanning: {}", grid[i][j]);
            let res = scan2(&grid, (i, j));
            println!("Res: {res}");
            if res > max {
                max = res;
            }
        }
    }

    println!("{max}");

    Ok(())

}
