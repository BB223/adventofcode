pub fn part_one(input: &str) -> Option<i64> {
    let coords: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut areas = vec![];
    for (i, coord) in coords.iter().enumerate() {
        for coord2 in coords.iter().skip(i) {
            let area = ((coord.0 - coord2.0).abs() + 1) * ((coord.1 - coord2.1).abs() + 1);
            areas.push(area);
        }
    }
    areas.sort();
    areas.reverse();
    Some(areas[0])
}

type Point = (f64, f64);

pub fn part_two(input: &str) -> Option<f64> {
    let coords: Vec<Point> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut areas = vec![];
    for (i, coord) in coords.iter().enumerate() {
        for coord2 in coords.iter().skip(i) {
            let (xmin, xmax) = (coord.0.min(coord2.0), coord.0.max(coord2.0));
            let (ymin, ymax) = (coord.1.min(coord2.1), coord.1.max(coord2.1));

            let rect_corners = [(xmin, ymin), (xmin, ymax), (xmax, ymin), (xmax, ymax)];
            if rect_corners
                .iter()
                .any(|corner| !point_in_polygon(*corner, &coords))
            {
                continue;
            }

            let rect_edges = [
                (rect_corners[0], rect_corners[1]),
                (rect_corners[1], rect_corners[3]),
                (rect_corners[3], rect_corners[2]),
                (rect_corners[2], rect_corners[0]),
            ];
            if rect_edges.iter().any(|edge| {
                for i in 0..coords.len() {
                    let p1 = coords[i];
                    let p2 = coords[(i + 1) % coords.len()];
                    if segments_intersect_strict(edge.0, edge.1, p1, p2) {
                        return true;
                    }
                }
                false
            }) {
                continue;
            }
            let area = ((coord.0 - coord2.0).abs() + 1.0) * ((coord.1 - coord2.1).abs() + 1.0);
            areas.push(area);
            /* if is_inside(&coords, other_corner) && is_inside(&coords, other_other_corner) {
                let area = ((coord.0 - coord2.0).abs() + 1) * ((coord.1 - coord2.1).abs() + 1);
                areas.push(area);
            } */
        }
    }
    areas.sort_by(f64::total_cmp);
    areas.reverse();
    Some(areas[0])
}

fn point_in_polygon(point: Point, poly: &[Point]) -> bool {
    let (px, py) = point;
    let mut inside = false;

    let n = poly.len();
    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];

        if point_on_segment(point, (x1, y1), (x2, y2)) {
            return true; // On edge or on vertex
        }

        // Check if edge crosses the horizontal ray
        let intersects = ((y1 > py) != (y2 > py)) && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1);

        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn point_on_segment(p: Point, a: Point, b: Point) -> bool {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;

    // Cross product to test collinearity
    let cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    if cross.abs() > f64::EPSILON {
        return false; // not collinear
    }

    // Check if p is within the bounding box of segment ab
    let within_x = px >= ax.min(bx) - f64::EPSILON && px <= ax.max(bx) + f64::EPSILON;
    let within_y = py >= ay.min(by) - f64::EPSILON && py <= ay.max(by) + f64::EPSILON;

    within_x && within_y
}

pub fn segments_intersect_strict(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = q1;
    let (x3, y3) = p2;
    let (x4, y4) = q2;

    let dx1 = x2 - x1;
    let dy1 = y2 - y1;
    let dx2 = x4 - x3;
    let dy2 = y4 - y3;

    let denom = dx1 * dy2 - dy1 * dx2;

    // Parallel or collinear → no strict intersection
    if denom.abs() < f64::EPSILON {
        return false;
    }

    // Solve for t and u parameters
    let t = ((x3 - x1) * dy2 - (y3 - y1) * dx2) / denom;
    let u = ((x3 - x1) * dy1 - (y3 - y1) * dx1) / denom;

    // Strict intersection requires:
    // 0 < t < 1 and 0 < u < 1
    // not touching endpoints (t==0, t==1, u==0, u==1)
    t > f64::EPSILON && t < 1.0 - f64::EPSILON && u > f64::EPSILON && u < 1.0 - f64::EPSILON
}

adventofcode::advent_of_code!(2025, 9, Some(50), Some(24.0));
