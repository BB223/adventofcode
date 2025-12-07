pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut llist: Vec<u32> = Vec::new();
    let mut rlist: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in lines {
        let mut nums = line.split_whitespace();
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        llist.push(left.parse::<u32>().unwrap());
        rlist.push(right.parse::<u32>().unwrap());
    }

    llist.sort();
    rlist.sort();

    for i in 0..llist.len() {
        sum += llist[i].abs_diff(rlist[i]);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut llist: Vec<u32> = Vec::new();
    let mut rlist: Vec<u32> = Vec::new();

    for line in lines {
        let mut nums = line.split_whitespace();
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        llist.push(left.parse::<u32>().unwrap());
        rlist.push(right.parse::<u32>().unwrap());
    }

    let sum = llist
        .iter()
        .map(|num| num * count_nums(&rlist, *num))
        .sum::<u32>();
    Some(sum)
}

fn count_nums(list: &[u32], num: u32) -> u32 {
    list.iter()
        .filter(|n| **n == num)
        .count()
        .try_into()
        .unwrap()
}

adventofcode::advent_of_code!(2024, 1, Some(11), Some(31));
