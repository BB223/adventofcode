pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim().split(',');
    let solution = ranges
        .flat_map(|range| {
            let numbers = range.split_once("-").unwrap();
            let lower: u64 = numbers.0.parse().unwrap();
            let upper: u64 = numbers.1.parse().unwrap();

            lower..upper + 1
        })
        .filter(|number| (number.checked_ilog10().unwrap_or(0) + 1).is_multiple_of(2))
        .filter(|number| {
            let num_str = number.to_string();
            let numbers = num_str.split_at(num_str.len() / 2);
            numbers.0 == numbers.1
        })
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.trim().split(',');
    let solution = ranges
        .flat_map(|range| {
            let numbers = range.split_once("-").unwrap();
            let lower: u64 = numbers.0.parse().unwrap();
            let upper: u64 = numbers.1.parse().unwrap();

            lower..upper + 1
        })
        .filter(|number| {
            let doubled = format!("{number}{number}");
            doubled[1..doubled.len() - 1].contains(&number.to_string())
        })
        .sum();

    Some(solution)
}

adventofcode::advent_of_code!(2025, 2, Some(1227775554), Some(4174379265));
