use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let quantity_of_shapes = lines.clone().filter(|d| d.is_empty()).count();
    let chunk_size = lines.clone().take_while(|d| !d.is_empty()).count() + 1;
    let shape_size = chunk_size - 2;

    let mut shapes = Vec::with_capacity(quantity_of_shapes);

    for (index, chunk) in (&lines
        .by_ref()
        .take(quantity_of_shapes * chunk_size)
        .chunks(chunk_size))
        .into_iter()
        .enumerate()
    {
        shapes.push(Vec::with_capacity(shape_size));
        // skip index line
        for line in chunk.skip(1).take(shape_size) {
            // build data structure for storing shapes
            shapes[index].push(line);
        }
    }
    dbg!(&shapes);

    let solution = lines
        .map(|tree| {
            let (region, quantity_of_each_shape) = tree.split_once(':').unwrap();
            let region: (usize, usize) = region
                .split_once('x')
                .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
                .unwrap();
            let quantity_of_each_shape = quantity_of_each_shape
                .split_whitespace()
                .flat_map(|d| d.parse().ok())
                .collect_vec();

            (region, quantity_of_each_shape)
        })
        .filter(|(region, quantity_of_each_shape)| {
            is_valid(*region, quantity_of_each_shape, &shapes)
        })
        .count();

    Some(solution)
}

/// takes the region, the number of shapes to fit and the shapes datastructure
fn is_valid(
    (width, height): (usize, usize),
    quantity_of_each_shape: &[i32],
    shapes: &[Vec<&str>],
) -> bool {
    dbg!(width, height);
    for (i, quantity) in quantity_of_each_shape
        .iter()
        .enumerate()
        .filter(|(_, quantity)| **quantity > 0)
    {
        dbg!(quantity);
        dbg!(&shapes[i]);
    }
    let num = quantity_of_each_shape.iter().sum::<i32>();
    dbg!(num);
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

adventofcode::advent_of_code!(2025, 12, Some(2), None);
