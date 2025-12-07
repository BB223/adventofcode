use std::{thread, time::Duration};

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut grid = vec![];
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut solution = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char != '@' {
                continue;
            }

            let mut stuff = vec![];
            for (k, l) in dirs {
                if k == 0 && l == 0 {
                    continue;
                }
                let dy = i as isize + k;
                let dx = j as isize + l;
                let (dy, dx) = (dy as usize, dx as usize);
                let to_check = grid.get(dy).and_then(|row| row.get(dx));
                stuff.push(to_check);
            }

            if stuff
                .iter()
                .filter_map(|d| *d)
                .filter(|d| **d == '@')
                .count()
                < 4
            {
                solution += 1;
            }
        }
    }

    Some(solution)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut grid = vec![];
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let original = grid.iter().flatten().filter(|c| **c == '@').count() as i32;

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut iteration;

    let mut removed = 1;
    while removed > 0 {
        removed = 0;
        iteration = grid.clone();
        print(&grid);
        thread::sleep(Duration::from_millis(250));
        for (i, row) in iteration.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char != '@' {
                    continue;
                }

                let mut stuff = vec![];
                for (k, l) in dirs {
                    if k == 0 && l == 0 {
                        continue;
                    }
                    let dy = i as isize + k;
                    let dx = j as isize + l;
                    let (dy, dx) = (dy as usize, dx as usize);
                    let to_check = iteration.get(dy).and_then(|row| row.get(dx));
                    stuff.push(to_check);
                }

                if stuff
                    .iter()
                    .filter_map(|d| *d)
                    .filter(|d| **d == '@')
                    .count()
                    < 4
                {
                    grid[i][j] = '.';
                    removed += 1;
                }
            }
        }
    }
    print(&grid);
    let new = grid.iter().flatten().filter(|c| **c == '@').count() as i32;

    Some(original - new)
}

fn braille(bits: u8) -> char {
    let code = 0x2800 + bits as u32;
    char::from_u32(code).unwrap()
}

fn print(map: &[Vec<char>]) {
    print!("\x1B[0;0H");
    map.iter().for_each(|line| {
        line.iter().for_each(|c| {
            if *c == '.' {
                print!("{}{}", braille(0), braille(0))
            } else {
                print!("{}{}", braille(0b11111111), braille(0b11111111))
            }
        });
        println!();
    });
}

adventofcode::advent_of_code!(2025, 4, Some(13), Some(43));
