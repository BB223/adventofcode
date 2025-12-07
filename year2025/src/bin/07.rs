use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    'outer: for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = (i + 1, j);
                break 'outer;
            }
        }
    }
    beam_down(&mut grid, start)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    'outer: for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = (i + 1, j);
                break 'outer;
            }
        }
    }
    let mut cache = HashMap::new();
    beam_down2(&grid, start, &mut cache)
}

fn beam_down(grid: &mut [Vec<char>], start: (usize, usize)) -> Option<u64> {
    if grid.get(start.0)?.get(start.1)? == &'|' {
        return None;
    }

    let x = start.1;
    let mut y = start.0;

    while grid.get(y)?.get(x)? != &'^' {
        if grid[y][x] == '|' {
            return None;
        }
        grid[y][x] = '|';
        y += 1;
    }
    let left = beam_down(grid, (y, x - 1)).unwrap_or(0);
    let right = beam_down(grid, (y, x + 1)).unwrap_or(0);
    Some(1 + left + right)
}

fn beam_down2(
    grid: &[Vec<char>],
    start: (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> Option<u64> {
    if let Some(x) = cache.get(&start) {
        return Some(*x);
    }
    let x = start.1;
    let mut y = start.0;

    while grid.get(y)?.get(x)? != &'^' {
        y += 1;
    }
    let left = beam_down2(grid, (y, x - 1), cache).unwrap_or(1);
    cache.insert((y, x - 1), left);
    let right = beam_down2(grid, (y, x + 1), cache).unwrap_or(1);
    cache.insert((y, x + 1), right);
    Some(left + right)
}

adventofcode::advent_of_code!(2025, 7, Some(21), Some(40));
