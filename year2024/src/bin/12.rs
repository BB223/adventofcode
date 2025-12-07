use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut garden: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let mut row: Vec<char> = Vec::with_capacity(line.len() + 2);
            row.push(' ');
            row.extend(line.chars());
            row.push(' ');
            row
        })
        .collect();

    let width = garden[0].len();
    let padding_row = vec![' '; width];

    garden.insert(0, padding_row.clone());
    garden.push(padding_row);
    let height = garden.len();

    let mut checked = HashMap::new();
    let mut solution = vec![];
    for (y, line) in garden.iter().enumerate().skip(1).take(height - 2) {
        for (x, plant_type) in line.iter().enumerate().skip(1).take(width - 2) {
            if checked.contains_key(&(y, x)) {
                continue;
            }
            let (region, fence) = fences(&garden, (y, x), *plant_type, &mut checked);
            solution.push(region * fence);
        }
    }
    Some(solution.iter().sum())
}

fn fences(
    garden: &[Vec<char>],
    position: (usize, usize),
    plant_type: char,
    checked: &mut HashMap<(usize, usize), bool>,
) -> (u64, u64) {
    if checked.get(&position).is_some() {
        return (0, 0);
    }
    checked.insert(position, true);
    let mut cur_fences = 0;
    let mut cur_region = 1;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for dir in dirs {
        let (y, x) = (position.0 as isize + dir.0, position.1 as isize + dir.1);
        if garden[y as usize][x as usize] == plant_type {
            let (new_region, new_fences) =
                fences(garden, (y as usize, x as usize), plant_type, checked);
            cur_region += new_region;
            cur_fences += new_fences;
        } else {
            cur_fences += 1;
        }
    }
    (cur_region, cur_fences)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut garden: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let mut row: Vec<char> = Vec::with_capacity(line.len() + 2);
            row.push(' ');
            row.extend(line.chars());
            row.push(' ');
            row
        })
        .collect();

    let width = garden[0].len();
    let padding_row = vec![' '; width];

    garden.insert(0, padding_row.clone());
    garden.push(padding_row);
    let height = garden.len();

    let mut checked = HashMap::new();
    let mut solution = vec![];
    for (y, line) in garden.iter().enumerate().skip(1).take(height - 2) {
        for (x, plant_type) in line.iter().enumerate().skip(1).take(width - 2) {
            if checked.contains_key(&(y, x)) {
                continue;
            }
            let mut garden2 = garden.clone();
            let (region, _) = fences2(&mut garden2, (y, x), *plant_type, &mut checked);
            let fence = fence_rows(&garden2, plant_type);
            let garden2 = transpose(garden2);
            let fence2 = fence_rows(&garden2, plant_type);
            solution.push(region * (fence + fence2));
        }
    }
    Some(solution.iter().sum())
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut out = vec![vec![' '; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            out[c][r] = grid[r][c];
        }
    }

    out
}

fn print(map: &[Vec<char>]) {
    // print!("\x1B[0;0H");
    map.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn fences2(
    garden: &mut [Vec<char>],
    position: (usize, usize),
    plant_type: char,
    checked: &mut HashMap<(usize, usize), bool>,
) -> (u64, u64) {
    if checked.get(&position).is_some() {
        return (0, 0);
    }
    checked.insert(position, true);
    let mut cur_fences = 0;
    let mut cur_region = 1;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for dir in dirs {
        let (y, x) = (position.0 as isize + dir.0, position.1 as isize + dir.1);
        if garden[y as usize][x as usize] == plant_type {
            let (new_region, new_fences) =
                fences2(garden, (y as usize, x as usize), plant_type, checked);
            cur_region += new_region;
            cur_fences += new_fences;
        } else {
            garden[y as usize][x as usize] = '+';
            cur_fences += 1;
        }
    }
    (cur_region, cur_fences)
}

fn fence_rows(garden: &[Vec<char>], plant_type: &char) -> u64 {
    let dirs = [(-1, 0), (1, 0)];
    let mut sum = 0;
    for (y, line) in garden.iter().enumerate() {
        let mut is_fence = false;
        for (x, c) in line.iter().enumerate() {
            if *c == '+' && !is_fence {
                let tmp = sum;
                for dir in dirs {
                    let (dy, dx) = (y as isize + dir.0, x as isize + dir.1);
                    if garden.get(dy as usize).and_then(|d| d.get(dx as usize)) == Some(plant_type)
                    {
                        sum += 1;
                    }
                }
                if tmp == sum && garden[y].get(x + 1) == Some(&'+') {
                    sum += 1;
                }
                is_fence = true;
            } else if *c != '+' {
                is_fence = false;
            } else if *c == '+' && is_fence {
                let mut tmp = 0;
                for dir in dirs {
                    let (dy, dx) = (y as isize + dir.0, x as isize + dir.1);
                    if garden.get(dy as usize).and_then(|d| d.get(dx as usize)) == Some(plant_type)
                    {
                        tmp += 1;
                    }
                }
                if tmp == 2 {
                    sum += 1;
                }
            }
        }
    }
    sum
}

adventofcode::advent_of_code!(2024, 12, Some(1930), Some(1206));
