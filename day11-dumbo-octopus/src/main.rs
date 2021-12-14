fn main() {
    let mut input = parse_input(include_str!("input.txt"));

    println!(
        "Part 1: {}",
        run_simulation(&mut input.clone(), Some(100)).0
    );
    println!("Part 2: {:?}", run_simulation(&mut input, None).1);
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

fn run_simulation(grid: &mut Grid, steps: Option<usize>) -> (usize, Option<usize>) {
    let mut flashes = 0;
    let mut flashed = Vec::new();
    let mut first_all_flash = None;
    let grid_length = grid.fields.len();

    for step in match steps {
        Some(steps) => 1..=steps,
        None => 1..=usize::MAX,
    } {
        for index in 0..grid_length {
            let (flashed_count, newly_flashed) = bump(grid, index);
            flashes += flashed_count;

            if let Some(mut newly_flashed) = newly_flashed {
                flashed.append(&mut newly_flashed);
            }
        }

        if steps.is_none() && flashed.len() == grid_length {
            first_all_flash = Some(step);
            break;
        }

        for (x, y) in flashed.iter() {
            let flashed_index = grid.index_from_coordinates(*x, *y);
            grid.fields[flashed_index] = 0
        }

        flashed.clear();
    }

    (flashes, first_all_flash)
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

#[cfg(test)]
const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_part1() {
    let mut parsed = parse_input(TEST_INPUT);

    assert_eq!(204, run_simulation(&mut parsed.clone(), Some(10)).0);
    assert_eq!(1656, run_simulation(&mut parsed, Some(100)).0);
}

#[test]
fn test_part2() {
    assert_eq!(
        Some(195),
        run_simulation(&mut parse_input(TEST_INPUT), None).1
    );
}
