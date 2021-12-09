fn main() {
    println!(
        "{}",
        risk_level_sum(&find_low_points(&parse_input(include_str!("input.txt"))))
    );
}

struct Grid {
    pub items: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    fn coordinates_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    #[inline]
    fn index_from_coordintes(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(4);
        if y > 0 {
            result.push(self.items[self.index_from_coordintes(x, y - 1)]);
        }
        if x < self.width - 1 {
            result.push(self.items[self.index_from_coordintes(x + 1, y)]);
        }
        if y < self.height - 1 {
            result.push(self.items[self.index_from_coordintes(x, y + 1)]);
        }
        if x > 0 {
            result.push(self.items[self.index_from_coordintes(x - 1, y)]);
        }

        result
    }
}

fn parse_input(input: &str) -> Grid {
    let mut width = 0;

    let items: Vec<_> = input
        .lines()
        .flat_map(|line| {
            width = line.len();

            line.chars()
                .filter_map(|char| char.to_digit(10).map(|num| num as u8))
        })
        .collect();

    Grid {
        height: items.len() / width,
        items,
        width,
    }
}

fn find_low_points(grid: &Grid) -> Vec<u8> {
    grid.items
        .iter()
        .enumerate()
        .filter_map(|(index, digit)| {
            let (x, y) = grid.coordinates_from_index(index);
            if grid.get_surrounding(x, y).iter().all(|s| digit < s) {
                Some(*digit)
            } else {
                None
            }
        })
        .collect()
}

fn risk_level_sum(low_points: &[u8]) -> u32 {
    low_points.iter().map(|point| *point as u32 + 1).sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn test() {
    assert_eq!(
        15,
        risk_level_sum(&find_low_points(&parse_input(TEST_INPUT)))
    );
}
