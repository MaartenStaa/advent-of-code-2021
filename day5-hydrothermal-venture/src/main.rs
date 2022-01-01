use std::ops::{Add, Div, Mul, Sub};

fn main() {
    let lines = parse(include_str!("input.txt"));

    let (grid, _) = fill_grid(
        &lines
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical())
            .collect::<Vec<_>>(),
    );
    println!("{}", grid.iter().filter(|value| **value >= 2).count());
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point(isize, isize);

impl Point {
    fn len(&self) -> f32 {
        ((self.0 * self.0 + self.1 * self.1) as f32).sqrt()
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

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point(
            ((self.0 as f32) / rhs) as isize,
            ((self.1 as f32) / rhs) as isize,
        )
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point(
            ((self.0 as f32) * rhs) as isize,
            ((self.1 as f32) * rhs) as isize,
        )
    }
}

#[derive(Debug, PartialEq)]
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
        let distance = delta.len().ceil() as usize;
        let delta_step = delta / distance as f32;

        (0..=distance)
            .map(|step| self.0 + delta_step * step as f32)
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

fn fill_grid(lines: &[&Line]) -> (Vec<usize>, usize) {
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
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical())
            .collect::<Vec<_>>(),
    );
    assert_eq!(5, grid.iter().filter(|value| **value >= 2).count());
}
