pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines();

    let safe = lines
        .into_iter()
        .filter(|report| {
            is_safe(
                report
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .count();
    Some(safe)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines();

    let safe = lines
        .into_iter()
        .filter(|report| {
            is_safe2(
                report
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .count();
    Some(safe)
}

fn is_safe(report: Vec<u32>) -> bool {
    report.windows(3).all(is_safe_win)
}

fn is_safe2(report: Vec<u32>) -> bool {
    let all = report.windows(3).all(is_safe_win);

    if all {
        return true;
    }

    for i in 0..report.len() {
        let mut check = report.clone();
        check.remove(i);
        if check.windows(3).all(is_safe_win) {
            return true;
        }
    }

    false
}

fn is_safe_win(win: &[u32]) -> bool {
    let diff = win[0].abs_diff(win[1]);
    if diff == 0 || diff > 3 {
        return false;
    }

    let diff = win[1].abs_diff(win[2]);
    if diff == 0 || diff > 3 {
        return false;
    }

    if win[0] < win[1] && win[1] > win[2] {
        return false;
    }

    if win[0] > win[1] && win[1] < win[2] {
        return false;
    }

    true
}

adventofcode::advent_of_code!(2024, 2, Some(2), Some(4));
