use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let rs: i32 = regex
        .captures_iter(input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum();
    Some(rs)
}

pub fn part_two(input: &str) -> Option<i32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut splits = input.split("don't()");

    let mut inp = splits.next().unwrap().to_owned();

    for i in splits {
        let mut dos = i.split("do()");
        dos.next();
        inp.push_str(&dos.fold(String::new(), |mut acc, x| {
            acc.push_str(x);
            acc
        }))
    }

    let rs: i32 = regex
        .captures_iter(&inp)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum();
    Some(rs)
}

adventofcode::advent_of_code!(2024, 3, Some(161), Some(48));
