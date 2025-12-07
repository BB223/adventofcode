pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut database = vec![];
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let range = line
            .split_once("-")
            .map(|(lower, upper)| (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap()))
            .unwrap();
        database.push(range);
    }
    let solution = lines
        .filter_map(|n| n.parse().ok())
        .filter(|n| {
            database
                .iter()
                .any(|(lower, upper)| lower <= n && n <= upper)
        })
        .count();
    Some(solution as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut database = vec![];
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let range = line
            .split_once("-")
            .map(|(lower, upper)| (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap()))
            .unwrap();
        database.push(range);
    }
    database.sort_by(|cur, next| cur.0.cmp(&next.0));
    let mut merged = vec![database[0]];
    for element in database.iter().skip(1) {
        let to_merge = element.0;
        let cur = merged.last_mut().unwrap();
        if to_merge <= cur.1 {
            if element.1 > cur.1 {
                cur.1 = element.1;
            }
        } else {
            merged.push(*element);
        }
    }
    let solution = merged.iter().map(|(lower, upper)| upper - lower + 1).sum();
    Some(solution)
}

adventofcode::advent_of_code!(2025, 5, Some(3), Some(14));
