pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|bank| select_max(bank, 2).parse::<u64>().ok())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|bank| select_max(bank, 12).parse::<u64>().ok())
            .sum(),
    )
}

fn select_max(batteries: &str, mut remaining: usize) -> String {
    let chars: Vec<char> = batteries.chars().collect();
    let mut result = String::with_capacity(remaining);

    let mut start = 0;

    while remaining > 0 {
        let end = chars.len() - remaining;
        let mut max_char = '\0';
        let mut max_pos = start;

        for (position, battery) in chars.iter().enumerate().take(end + 1).skip(start) {
            if *battery > max_char {
                max_char = *battery;
                max_pos = position;
            }
        }

        result.push(max_char);
        start = max_pos + 1;
        remaining -= 1;
    }

    result
}

adventofcode::advent_of_code!(2025, 3, Some(357), Some(3121910778619));
