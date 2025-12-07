#[derive(Debug, Clone)]
enum Operations {
    Add,
    Mul,
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut nums: Vec<Vec<u64>> = vec![vec![]; grid[0].len()];
    let mut ops: Vec<Operations> = vec![Operations::Mul; grid[0].len()];

    for line in grid {
        for (i, element) in line.iter().enumerate() {
            if let Ok(num) = element.parse::<u64>() {
                nums[i].push(num);
            } else if *element == "+" {
                ops[i] = Operations::Add;
            }
        }
    }

    let solution = nums
        .iter()
        .enumerate()
        .filter_map(|(i, num)| match ops[i] {
            Operations::Add => num.iter().copied().reduce(|acc, n| acc + n),
            Operations::Mul => num.iter().copied().reduce(|acc, n| acc * n),
        })
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let ops = lines.next_back().unwrap().split_whitespace();
    let groups: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let solution = (0..groups[0].len())
        .map(|i| groups.iter().map(|g| g[i]).collect::<String>())
        .map(|c| c.trim().parse().unwrap_or(0))
        .collect::<Vec<u64>>()
        .split(|n| *n == 0)
        .zip(ops)
        .filter_map(|(p, op)| {
            if op == "+" {
                p.iter().copied().reduce(|acc, n| acc + n)
            } else {
                p.iter().copied().reduce(|acc, n| acc * n)
            }
        })
        .sum();

    Some(solution)
}

adventofcode::advent_of_code!(2025, 6, Some(4277556), Some(3263827));
