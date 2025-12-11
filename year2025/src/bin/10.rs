use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines();
    let mut solution = 0;
    let first = Regex::new(r"\[[\.#]+]").unwrap();
    let regex = Regex::new(r"\(\d(?:,\d)*\)*").unwrap();

    for line in lines {
        let result = first.find(line);

        let mat = rem_first_and_last(result.unwrap().as_str());

        let result = regex.captures_iter(line);
        let buttons: Vec<&str> = result
            .map(|c| c.get_match().as_str())
            .map(rem_first_and_last)
            .collect();

        let min_presses = brute_force(mat, &buttons, 1);
        solution += min_presses.unwrap();
    }
    Some(solution)
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn brute_force(solution: &str, buttons: &[&str], presses: usize) -> Option<usize> {
    if presses == 10 {
        return None;
    }

    if buttons
        .iter()
        .combinations_with_replacement(presses)
        .any(|combination| {
            let mut lights = HashMap::new();
            for button in combination {
                for c in button.chars() {
                    if let Some(n) = lights.get(&c) {
                        lights.insert(c, n + 1);
                    } else {
                        lights.insert(c, 1);
                    }
                }
            }
            solution.chars().enumerate().all(|(i, c)| {
                let indx = char::from_digit(i as u32, 10).unwrap();
                if c == '.' {
                    lights.get(&indx).unwrap_or(&0) % 2 == 0
                } else {
                    lights.get(&indx).unwrap_or(&0) % 2 == 1
                }
            })
        })
    {
        return Some(presses);
    }

    brute_force(solution, buttons, presses + 1)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut solution = 0;
    let first = Regex::new(r"\{\d+(?:,\d+)*\}").unwrap();
    let regex = Regex::new(r"\(\d(?:,\d)*\)*").unwrap();

    for line in lines {
        let result = first.find(line);
        let joltage_requirements = rem_first_and_last(result.unwrap().as_str());

        let result = regex.captures_iter(line);
        let buttons: Vec<&str> = result
            .map(|c| c.get_match().as_str())
            .map(rem_first_and_last)
            .collect();

        let max = joltage_requirements
            .split(',')
            .map(|d| d.parse().unwrap())
            .max()
            .unwrap();
        // let mut cache = HashMap::new();
        // let min_presses = bfs(HashMap::new(), 0, &buttons, joltage_requirements);
        let min_presses = brute_force2(joltage_requirements, &buttons, max);
        solution += min_presses.unwrap();
    }
    Some(solution as i32)
}

fn bfs(
    current: HashMap<char, i32>,
    depth: i32,
    buttons: &[&str],
    joltage_requirements: &str,
) -> Option<i32> {
    let mut q = VecDeque::new();
    q.push_back(current);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        if joltage_requirements.split(',').enumerate().all(|(i, c)| {
            let indx = char::from_digit(i as u32, 10).unwrap();
            let c_num = c.parse().unwrap();
            v.get(&indx).unwrap_or(&0) == &c_num
        }) {
            return Some(depth);
        }
        for button in buttons {
            let mut cloned = v.clone();
            for c in button.chars() {
                if let Some(n) = cloned.get(&c) {
                    cloned.insert(c, n + 1);
                } else {
                    cloned.insert(c, 1);
                }
            }
            if !joltage_requirements.split(',').enumerate().any(|(i, c)| {
                let indx = char::from_digit(i as u32, 10).unwrap();
                let c_num = c.parse().unwrap();
                cloned.get(&indx).unwrap_or(&0) > &c_num
            }) {
                q.push_back(cloned);
            }
        }
    }
    None
}

fn brute_force2(solution: &str, buttons: &[&str], presses: usize) -> Option<usize> {
    if presses == 100 {
        return None;
    }
    dbg!(
        buttons
            .iter()
            .combinations_with_replacement(presses)
            .map(|combination| {
                let mut v: Vec<String> = combination.iter().map(|s| s.to_string()).collect();
                v.sort();
                v
            })
            .unique()
            .count()
    );
    dbg!(
        buttons
            .iter()
            .combinations_with_replacement(presses)
            .map(|combination| {
                let mut v: Vec<String> = combination.iter().map(|s| s.to_string()).collect();
                v.sort();
                v
            })
            .count()
    );
    return None;

    // if buttons
    //     .iter()
    //     .combinations_with_replacement(presses)
    //     .map(|combination| {
    //         let mut v: Vec<String> = combination.iter().map(|s| s.to_string()).collect();
    //         v.sort();
    //         v
    //     })
    //     .unique()
    //     .par_bridge()
    //     .any(|normilized| test_combination(normilized, solution))
    // {
    //     return Some(presses);
    // }

    // brute_force2(solution, buttons, presses + 1)
}

fn test_combination(
    combination: Vec<String>,
    joltage_requirements: &str,
    // cache: &mut HashMap<Vec<String>, bool>,
) -> bool {
    // if let Some(b) = cache.get(&combination) {
    //     return *b;
    // }
    let mut lights = HashMap::new();
    for button in &combination {
        for c in button.chars() {
            if let Some(n) = lights.get(&c) {
                lights.insert(c, n + 1);
            } else {
                lights.insert(c, 1);
            }
            let to_big = joltage_requirements.split(',').enumerate().any(|(i, c)| {
                let indx = char::from_digit(i as u32, 10).unwrap();
                let c_num = c.parse().unwrap();
                lights.get(&indx).unwrap_or(&0) > &c_num
            });
            if to_big {
                // cache.insert(combination, false);
                return false;
            }
        }
    }
    let b = joltage_requirements.split(',').enumerate().all(|(i, c)| {
        let indx = char::from_digit(i as u32, 10).unwrap();
        let c_num = c.parse().unwrap();
        lights.get(&indx).unwrap_or(&0) == &c_num
    });
    // cache.insert(combination, b);
    b
}

adventofcode::advent_of_code!(2025, 10, Some(7), Some(33));
