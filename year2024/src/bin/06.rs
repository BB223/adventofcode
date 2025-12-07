pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = vec![];
    input.lines().for_each(|l| grid.push(l.chars().collect()));
    let mut start: (usize, usize) = (500000, 500000);

    for (i, item) in grid.iter().enumerate() {
        for (j, itemj) in item.iter().enumerate() {
            match itemj {
                '^' | 'v' | '>' | '<' => start = (i, j),
                _ => continue,
            }
        }
    }

    run(start, &mut grid);

    let res = grid
        .iter()
        .map(|v| v.iter().filter(|c| **c == 'X').count())
        .sum::<usize>();

    Some(res)
}

fn run(position: (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (mut nexti, mut nextj) = match grid[position.0][position.1] {
        '>' => (position.0, position.1 + 1),
        'v' => (position.0 + 1, position.1),
        '<' => (position.0, position.1 - 1),
        '^' => (position.0 - 1, position.1),
        _ => panic!("ups"),
    };
    if grid.get(nexti).is_none_or(|g| g.get(nextj).is_none()) {
        grid[position.0][position.1] = 'X';
        return;
    }

    if grid[nexti][nextj] == '#' {
        grid[position.0][position.1] = match grid[position.0][position.1] {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => panic!("ups"),
        };
        nexti = position.0;
        nextj = position.1;
    } else {
        grid[nexti][nextj] = grid[position.0][position.1];
        grid[position.0][position.1] = 'X';
    }

    run((nexti, nextj), grid);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

adventofcode::advent_of_code!(2024, 6, Some(41), Some(6));
