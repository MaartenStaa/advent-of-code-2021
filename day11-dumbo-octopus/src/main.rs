use std::vec;

fn main() {
    println!(
        "{}",
        run_simulation(&mut parse_input(include_str!("input.txt")), 100)
    );
}

#[derive(Debug, Clone)]
struct Grid {
    fields: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn coordinates_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    #[inline]
    fn index_from_coordinates(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_surrounding_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        if y > 0 {
            result.push((x, y - 1));
        }
        if y > 0 && x < self.width - 1 {
            result.push((x + 1, y - 1));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if x < self.width - 1 && y < self.height - 1 {
            result.push((x + 1, y + 1));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        if x > 0 && y < self.height - 1 {
            result.push((x - 1, y + 1));
        }
        if x > 0 {
            result.push((x - 1, y));
        }
        if x > 0 && y > 0 {
            result.push((x - 1, y - 1));
        }

        result
    }
}

fn parse_input(input: &str) -> Grid {
    let mut width = 0;
    let fields: Vec<_> = input
        .lines()
        .flat_map(|line| {
            width = line.len();

            line.chars()
                .filter_map(|char| char.to_digit(10).map(|num| num as u8))
        })
        .collect();

    Grid {
        height: fields.len() / width,
        fields,
        width,
    }
}

fn run_simulation(grid: &mut Grid, steps: usize) -> usize {
    let mut flashes = 0;
    let mut flashed = Vec::new();
    let grid_length = grid.fields.len();

    for _ in 0..steps {
        for index in 0..grid_length {
            let (flashed_count, newly_flashed) = bump(grid, index);
            flashes += flashed_count;

            if let Some(mut newly_flashed) = newly_flashed {
                flashed.append(&mut newly_flashed);
            }
        }

        for (x, y) in flashed.iter() {
            let flashed_index = grid.index_from_coordinates(*x, *y);
            grid.fields[flashed_index] = 0
        }

        flashed.clear();
    }

    flashes
}

fn bump(grid: &mut Grid, index: usize) -> (usize, Option<Vec<(usize, usize)>>) {
    // If it's already 10 (= flashed), nothing to do
    if grid.fields[index] == 10 {
        return (0, None);
    }

    grid.fields[index] += 1;
    if grid.fields[index] < 10 {
        return (0, None);
    }

    let (x, y) = grid.coordinates_from_index(index);
    let mut flashes = 1;
    let mut flashed = vec![(x, y)];
    for (x, y) in grid.get_surrounding_indices(x, y) {
        let (flashed_count, newly_flashed) = bump(grid, grid.index_from_coordinates(x, y));
        flashes += flashed_count;

        if let Some(mut newly_flashed) = newly_flashed {
            flashed.append(&mut newly_flashed);
        }
    }

    (flashes, Some(flashed))
}

#[test]
fn test_part1() {
    let mut parsed = parse_input(
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );

    assert_eq!(204, run_simulation(&mut parsed.clone(), 10));
    assert_eq!(1656, run_simulation(&mut parsed, 100));
}
