use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut checked = HashMap::new();
    let mut solution = vec![];
    for (y, line) in garden.iter().enumerate() {
        for (x, plant_type) in line.iter().enumerate() {
            if checked.contains_key(&(y as isize, x as isize)) {
                continue;
            }
            let (region, fence) =
                fences(&garden, (y as isize, x as isize), *plant_type, &mut checked);
            solution.push(region * fence);
        }
    }
    Some(solution.iter().sum())
}

fn fences(
    garden: &[Vec<char>],
    position: (isize, isize),
    plant_type: char,
    checked: &mut HashMap<(isize, isize), bool>,
) -> (u64, u64) {
    if checked.get(&position).is_some() {
        return (0, 0);
    }
    checked.insert(position, true);
    let mut cur_fences = 0;
    let mut cur_region = 1;
    let (y, x) = position;
    if garden.get_isize(y - 1).and_then(|g| g.get_isize(x)) == Some(&plant_type) {
        let (new_region, new_fences) = fences(garden, (y - 1, x), plant_type, checked);
        cur_region += new_region;
        cur_fences += new_fences;
    } else {
        cur_fences += 1;
    }
    if garden.get_isize(y + 1).and_then(|g| g.get_isize(x)) == Some(&plant_type) {
        let (new_region, new_fences) = fences(garden, (y + 1, x), plant_type, checked);
        cur_region += new_region;
        cur_fences += new_fences;
    } else {
        cur_fences += 1;
    }
    if garden.get_isize(y).and_then(|g| g.get_isize(x - 1)) == Some(&plant_type) {
        let (new_region, new_fences) = fences(garden, (y, x - 1), plant_type, checked);
        cur_region += new_region;
        cur_fences += new_fences;
    } else {
        cur_fences += 1;
    }
    if garden.get_isize(y).and_then(|g| g.get_isize(x + 1)) == Some(&plant_type) {
        let (new_region, new_fences) = fences(garden, (y, x + 1), plant_type, checked);
        cur_region += new_region;
        cur_fences += new_fences;
    } else {
        cur_fences += 1;
    }
    (cur_region, cur_fences)
}
trait GetIsize<T> {
    fn get_isize(&self, idx: isize) -> Option<&T>;
}

impl<T> GetIsize<T> for [T] {
    fn get_isize(&self, idx: isize) -> Option<&T> {
        if idx < 0 {
            None
        } else {
            self.get(idx as usize)
        }
    }
}

impl<T> GetIsize<T> for Vec<T> {
    fn get_isize(&self, idx: isize) -> Option<&T> {
        self.as_slice().get_isize(idx)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

adventofcode::advent_of_code!(2024, 12, Some(1930), None);
