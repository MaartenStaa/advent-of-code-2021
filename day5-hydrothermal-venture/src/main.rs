use std::ops::{Add, Div, Mul, Sub};

fn main() {
    let lines = parse(include_str!("input.txt"));

    let (grid, _) = fill_grid(
        &lines
            .clone()
            .into_iter()
            .filter(|line| line.is_horizontal() || line.is_vertical())
            .collect::<Vec<_>>(),
    );
    println!(
        "Part 1: {}",
        grid.iter().filter(|value| **value >= 2).count()
    );

    let (grid, _) = fill_grid(&lines);
    println!(
        "Part 2: {}",
        grid.iter().filter(|value| **value >= 2).count()
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point(isize, isize);

impl Point {
    fn len(&self) -> isize {
        self.0.abs().max(self.1.abs())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div<isize> for Point {
    type Output = Point;

    fn div(self, rhs: isize) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn is_vertical(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    fn points(&self) -> Vec<Point> {
        let delta = self.1 - self.0;
        let distance = delta.len();
        if distance == 0 {
            return vec![self.0];
        }

        let delta_step = delta / distance;

        (0..=distance)
            .map(|step| self.0 + delta_step * step)
            .collect()
    }
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(" -> ")
                .map(|(from, to)| (from.split_once(','), to.split_once(',')))
                .and_then(|(from, to)| match (from, to) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                })
                .and_then(|((from_x, from_y), (to_x, to_y))| {
                    Some(Line(
                        Point(from_x.parse().ok()?, from_y.parse().ok()?),
                        Point(to_x.parse().ok()?, to_y.parse().ok()?),
                    ))
                })
        })
        .collect()
}

fn fill_grid(lines: &[Line]) -> (Vec<usize>, usize) {
    let width = lines
        .iter()
        .flat_map(|line| [line.0 .0, line.1 .0])
        .max()
        .unwrap_or_default() as usize
        + 1;
    let height = lines
        .iter()
        .flat_map(|line| [line.0 .1, line.1 .1])
        .max()
        .unwrap_or_default() as usize
        + 1;

    let mut grid = Vec::from_iter(std::iter::repeat(0).take(width * height));
    for line in lines {
        for Point(x, y) in line.points() {
            grid[y as usize * width + x as usize] += 1;
        }
    }

    (grid, width)
}

#[test]
fn test_part1() {
    let lines = parse(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );

    let (grid, _) = fill_grid(
        &lines
            .into_iter()
            .filter(|line| line.is_horizontal() || line.is_vertical())
            .collect::<Vec<_>>(),
    );
    assert_eq!(5, grid.iter().filter(|value| **value >= 2).count());
}

#[test]
fn test_covered_points() {
    // Horizontal
    assert_eq!(
        vec![Point(15, 2), Point(16, 2), Point(17, 2), Point(18, 2)],
        Line(Point(15, 2), Point(18, 2)).points()
    );

    // Vertical
    assert_eq!(
        vec![Point(1, 2), Point(1, 3), Point(1, 4)],
        Line(Point(1, 2), Point(1, 4)).points()
    );

    // Vertical going up
    assert_eq!(
        vec![Point(1, 4), Point(1, 3), Point(1, 2)],
        Line(Point(1, 4), Point(1, 2)).points()
    );

    // Diagonal
    assert_eq!(
        vec![Point(1, 1), Point(2, 2), Point(3, 3), Point(4, 4)],
        Line(Point(1, 1), Point(4, 4)).points()
    );

    // One point
    assert_eq!(vec![Point(1, 1)], Line(Point(1, 1), Point(1, 1)).points());
}

#[test]
fn test_part2() {
    let lines = parse(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );

    let (grid, _) = fill_grid(&lines);
    assert_eq!(12, grid.iter().filter(|value| **value >= 2).count());
}
