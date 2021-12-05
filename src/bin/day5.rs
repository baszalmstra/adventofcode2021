use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

#[derive(Clone, Copy, Debug)]
struct Line(Point, Point);

fn parse(input: &str) -> anyhow::Result<Vec<Line>> {
    input
        .lines()
        .map(|line| -> anyhow::Result<Line> {
            let (left, right) = line.split_once("->").unwrap();
            let (x1, y1) = left.split_once(",").unwrap();
            let (x2, y2) = right.split_once(",").unwrap();
            Ok(Line(
                Point(x1.trim().parse()?, y1.trim().parse()?),
                Point(x2.trim().parse()?, y2.trim().parse()?),
            ))
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let lines = parse(std::fs::read_to_string("inputs/day5/input")?.as_str())?;

    println!("Solution 1: {}", find_overlapping_points(&lines, false));
    println!("Solution 2: {}", find_overlapping_points(&lines, true));

    Ok(())
}

fn find_overlapping_points(lines: &[Line], consider_diagonals: bool) -> usize {
    let mut points_visited: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        if line.0 .1 == line.1 .1 {
            for x in line.0 .0.min(line.1 .0)..=line.0 .0.max(line.1 .0) {
                let point = Point(x, line.0 .1);
                *points_visited.entry(point).or_insert(0) += 1;
            }
        } else if line.0 .0 == line.1 .0 {
            for y in line.0 .1.min(line.1 .1)..=line.0 .1.max(line.1 .1) {
                let point = Point(line.0 .0, y);
                *points_visited.entry(point).or_insert(0) += 1;
            }
        } else if consider_diagonals {
            let dx = (line.1 .0 - line.0 .0).signum();
            let dy = (line.1 .1 - line.0 .1).signum();
            let mut point = line.0;
            while point != line.1 {
                *points_visited.entry(point).or_insert(0) += 1;
                point = Point(point.0 + dx, point.1 + dy);
            }
            *points_visited.entry(point).or_insert(0) += 1;
        }
    }
    points_visited
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}
