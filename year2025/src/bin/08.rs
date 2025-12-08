use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let mut distances = HashMap::new();
    let mut circuites = vec![];
    let junktion_boxes = input
        .lines()
        .flat_map(|junktion_box| {
            junktion_box
                .split(",")
                .flat_map(|coord| coord.parse::<i64>().ok())
                .collect_tuple::<(i64, i64, i64)>()
        })
        .collect_vec();
    for (i, (x, y, z)) in junktion_boxes.iter().enumerate() {
        for (j, (a, b, c)) in junktion_boxes.iter().enumerate().skip(i + 1) {
            let distance = f64::sqrt(((x - a).pow(2) + (y - b).pow(2) + (z - c).pow(2)) as f64);
            distances.insert((i, j), distance);
        }
    }
    distances
        .iter()
        .sorted_by(|(_, v), (_, v2)| v.total_cmp(v2))
        .take(1000)
        .for_each(|(k, _)| {
            if !add_circuites(&mut circuites, *k) {
                circuites.push(vec![k.0, k.1]);
            }
        });
    circuites = merge(circuites);
    circuites.sort_by_key(|next| std::cmp::Reverse(next.len()));
    let solution = circuites
        .iter()
        .take(3)
        .map(|circuite| circuite.len())
        .product::<usize>();
    Some(solution as u64)
}

fn add_circuites(circuites: &mut [Vec<usize>], k: (usize, usize)) -> bool {
    for (i, circuite) in circuites.iter().cloned().enumerate() {
        let k0 = circuite.iter().find(|key| **key == k.0);
        let k1 = circuite.iter().find(|key| **key == k.1);
        if k0.is_some() && k1.is_some() {
            return true;
        }
        if k0.is_some() {
            circuites[i].push(k.1);
            return true;
        }
        if k1.is_some() {
            circuites[i].push(k.0);
            return true;
        }
    }

    false
}

fn merge(mut circuites: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut changed = true;
    while changed {
        changed = false;
        'outer: for i in 0..circuites.len() {
            for j in (i + 1)..circuites.len() {
                if overlaps(&circuites[i], &circuites[j]) {
                    let mut merged = circuites[i].clone();
                    merged.extend(circuites[j].iter());
                    merged.sort();
                    merged.dedup();
                    circuites[i] = merged;
                    circuites.remove(j);

                    changed = true;
                    break 'outer;
                }
            }
        }
    }

    circuites
}

fn overlaps(circuite_a: &[usize], circuite_b: &[usize]) -> bool {
    for a in circuite_a {
        if circuite_b.contains(a) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut distances = HashMap::new();
    let junktion_boxes = input
        .lines()
        .flat_map(|junktion_box| {
            junktion_box
                .split(",")
                .flat_map(|coord| coord.parse::<i64>().ok())
                .collect_tuple::<(i64, i64, i64)>()
        })
        .collect_vec();
    let mut circuites = vec![];
    for i in 0..junktion_boxes.len() {
        circuites.push(vec![i]);
    }
    for (i, (x, y, z)) in junktion_boxes.iter().enumerate() {
        for (j, (a, b, c)) in junktion_boxes.iter().enumerate().skip(i + 1) {
            let distance = f64::sqrt(((x - a).pow(2) + (y - b).pow(2) + (z - c).pow(2)) as f64);
            distances.insert((i, j), distance);
        }
    }
    let ding = distances
        .iter()
        .sorted_by(|(_, v), (_, v2)| v.total_cmp(v2))
        .collect_vec();

    let mut last = (0, 0);
    for (k, _) in ding {
        if !add_circuites(&mut circuites, *k) {
            circuites.push(vec![k.0, k.1]);
        }
        circuites = merge(circuites);
        if circuites.len() == 1 {
            last = *k;
            break;
        }
    }
    let solution = junktion_boxes[last.0].0 * junktion_boxes[last.1].0;
    Some(solution as u64)
}

adventofcode::advent_of_code!(2025, 8, Some(40), Some(25272));
