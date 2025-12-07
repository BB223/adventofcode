pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut dial = 50;
    let mut solution = 0;

    for line in lines {
        let chars = line.split_at(1);
        let char = chars.0;
        let times: i32 = chars.1.parse().unwrap();
        dial = if char == "R" {
            (dial + times).rem_euclid(100)
        } else {
            (dial - times).rem_euclid(100)
        };
        if dial == 0 {
            solution += 1;
        }
    }

    Some(solution)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut dial = 50;
    let mut solution = 0;

    for line in lines {
        let chars = line.split_at(1);
        let char = chars.0;
        let times: i32 = chars.1.parse().unwrap();
        dial = if char == "R" {
            let passes = times / 100;
            let rotation = dial + (times - passes * 100);
            solution += passes;
            if rotation > 100 {
                solution += 1;
            }
            rotation.rem_euclid(100)
        } else {
            let passes = times / 100;
            let rotation = dial - (times - passes * 100);
            solution += passes;
            if rotation < 0 && dial != 0 {
                solution += 1;
            }
            rotation.rem_euclid(100)
        };
        if dial == 0 {
            solution += 1;
        }
    }

    Some(solution)
}

adventofcode::advent_of_code!(2025, 1, Some(3), Some(6));
