use std::collections::{HashMap, VecDeque};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::{
    ThreadPoolBuilder,
    iter::{ParallelBridge, ParallelIterator},
};
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines();

    let lights_regex = Regex::new(r"\[[\.#]+\]").unwrap();
    let button_regex = Regex::new(r"\(\d(?:,\d)*\)*").unwrap();

    let solution = lines
        .clone()
        .par_bridge()
        .progress_count(lines.count() as u64)
        .map(|line| {
            let light_requirements: Vec<char> = lights_regex
                .find(line)
                .map(|matches| matches.as_str())
                .map(remove_parentheses)
                .unwrap()
                .chars()
                .collect();

            let buttons: Vec<Button> = button_regex
                .captures_iter(line)
                .map(|captures| captures.get_match().as_str())
                .map(remove_parentheses)
                .map(|button_str| {
                    button_str
                        .chars()
                        .flat_map(|char| char.to_digit(10))
                        .map(|digit| digit as usize)
                        .collect()
                })
                .collect();
            (buttons, light_requirements)
        })
        .map(|(buttons, light_requirements)| brute_force(&buttons, &light_requirements, 0).unwrap())
        .sum();
    Some(solution)
}

type Button = Vec<usize>;

fn remove_parentheses(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn brute_force(buttons: &[Button], light_requirements: &[char], presses: usize) -> Option<usize> {
    if presses == 10 {
        return None;
    }

    if buttons
        .iter()
        .combinations_with_replacement(presses)
        .any(|combination| {
            let mut lights = vec![0; light_requirements.len()];
            for button in combination {
                for index in button {
                    lights[*index] += 1;
                }
            }
            light_requirements.iter().enumerate().all(|(i, c)| {
                if *c == '.' {
                    lights[i] % 2 == 0
                } else {
                    lights[i] % 2 == 1
                }
            })
        })
    {
        return Some(presses);
    }

    brute_force(buttons, light_requirements, presses + 1)
}

pub fn part_two(input: &str) -> Option<i32> {
    // example input
    //
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    //
    let pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();
    let lines = input.lines();

    let joltage_regex = Regex::new(r"\{\d+(?:,\d+)*\}").unwrap();
    let button_regex = Regex::new(r"\(\d(?:,\d)*\)*").unwrap();

    let solution = pool.install(|| {
        lines
            .clone()
            .par_bridge()
            .progress_count(lines.count() as u64)
            .map(|line| {
                let joltage_requirements: Vec<i32> = joltage_regex
                    .find(line)
                    .map(|matches| matches.as_str())
                    .map(remove_parentheses)
                    .unwrap()
                    .split(',')
                    .flat_map(|c| c.parse().ok())
                    .collect();

                let buttons: Vec<Button> = button_regex
                    .captures_iter(line)
                    .map(|captures| captures.get_match().as_str())
                    .map(remove_parentheses)
                    .map(|button_str| {
                        button_str
                            .chars()
                            .flat_map(|char| char.to_digit(10))
                            .map(|digit| digit as usize)
                            .collect()
                    })
                    .collect();
                (buttons, joltage_requirements)
            })
            .map(|(buttons, joltage_requirements)| bfs(&buttons, &joltage_requirements))
            .sum()
    });
    Some(solution)
}

fn bfs(buttons: &[Button], joltage_requirements: &[i32]) -> i32 {
    let mut queue = VecDeque::new();
    let mut explored = HashMap::new();
    explored.insert(joltage_requirements.to_vec(), joltage_requirements.to_vec());
    queue.push_back(joltage_requirements.to_vec());
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.iter().all(|c| *c == 0) {
            break;
        }
        for button in buttons {
            let mut next = current.clone();
            for index in button {
                next[*index] -= 1;
            }
            if !explored.contains_key(&next) {
                explored.insert(next.clone(), current.clone());
                if next.iter().all(|joltage| *joltage >= 0) {
                    queue.push_back(next);
                }
            }
        }
    }
    let mut p = vec![0; joltage_requirements.len()];

    let mut steps = 0;
    while p != joltage_requirements {
        steps += 1;
        p = explored.get(&p).unwrap().to_vec();
    }

    steps
}

adventofcode::advent_of_code!(2025, 10, Some(7), Some(33));
