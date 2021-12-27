use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        simulate_until_stable(&mut parse(include_str!("input.txt")))
    );
}

fn simulate_until_stable(map: &mut Map) -> usize {
    let mut step = 1;
    loop {
        let any_moved_east = move_cucumbers(map, Cucumber::East);
        let any_moved_south = move_cucumbers(map, Cucumber::South);

        if !any_moved_east && !any_moved_south {
            break step;
        }

        step += 1;
    }
}

fn move_cucumbers(map: &mut Map, cucumber_type: Cucumber) -> bool {
    let to_move: Vec<_> = map
        .hm
        .iter()
        .filter(|(_, cucumber)| **cucumber == cucumber_type)
        .filter_map(|((x, y), entry)| {
            let new_position = entry.new_position(map, (*x, *y));
            if !map.hm.contains_key(&new_position) {
                Some(((*x, *y), new_position))
            } else {
                None
            }
        })
        .collect();
    let any_moved = !to_move.is_empty();

    for (move_from, move_to) in to_move {
        if let Some(cucumber) = map.hm.remove(&move_from) {
            map.hm.insert(move_to, cucumber);
        }
    }

    any_moved
}

#[derive(Debug, Clone, PartialEq)]
enum Cucumber {
    East,
    South,
}

impl Cucumber {
    fn new_position(&self, map: &Map, (x, y): Position) -> Position {
        match self {
            Cucumber::East => ((x + 1) % map.width, y),
            Cucumber::South => (x, (y + 1) % map.height),
        }
    }
}

type Position = (usize, usize);

struct Map {
    hm: HashMap<Position, Cucumber>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Map {
    let mut hm = HashMap::new();
    let height = input.lines().count();
    let lines = input.lines();
    let mut width = 0;

    for (y, line) in lines.enumerate() {
        width = line.len();

        for (x, c) in line.chars().enumerate() {
            if c == '>' {
                hm.insert((x, y), Cucumber::East);
            } else if c == 'v' {
                hm.insert((x, y), Cucumber::South);
            }
        }
    }

    Map { hm, width, height }
}

#[test]
fn test() {
    let mut map = parse(
        "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
    );

    assert_eq!(58, simulate_until_stable(&mut map));
}
