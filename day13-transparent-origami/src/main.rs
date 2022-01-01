use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let (set, instructions) = parse(input);
    let after_one_fold = execute_fold(&set, &instructions[0]);

    println!("Part 1: {}", after_one_fold.len());

    let mut set = set;
    for instruction in instructions {
        set = execute_fold(&set, &instruction);
    }

    println!("{}", as_text(&set));
}

#[derive(Clone, Debug, PartialEq)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<FoldInstruction>) {
    let mut set = HashSet::new();
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        if let Some((x, y)) = line.split_once(',') {
            if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                set.insert((x, y));
            }
        }
    }

    // The rest are fold instructions.
    let mut instructions = vec![];
    for line in lines {
        if let Some(line) = line.strip_prefix("fold along ") {
            if let Some((left, right)) = line.split_once('=') {
                match (left, right.parse()) {
                    ("x", Ok(x)) => instructions.push(FoldInstruction::X(x)),
                    ("y", Ok(y)) => instructions.push(FoldInstruction::Y(y)),
                    _ => {}
                }
            }
        }
    }

    (set, instructions)
}

fn execute_fold(
    input: &HashSet<(usize, usize)>,
    instruction: &FoldInstruction,
) -> HashSet<(usize, usize)> {
    input
        .iter()
        .map(|&(x, y)| {
            (
                match *instruction {
                    FoldInstruction::X(at) if x < at => x,
                    FoldInstruction::X(at) => at - (x - at),
                    _ => x,
                },
                match *instruction {
                    FoldInstruction::Y(at) if y < at => y,
                    FoldInstruction::Y(at) => at - (y - at),
                    _ => y,
                },
            )
        })
        .collect()
}

fn as_text(set: &HashSet<(usize, usize)>) -> String {
    let (width, height) = set.iter().fold((0, 0), |(width, height), &(x, y)| {
        (width.max(x + 1), height.max(y + 1))
    });

    let mut result = Vec::from_iter(std::iter::repeat('.').take(width * height));
    for &(x, y) in set {
        result[y * width + x] = '#';
    }

    result
        .chunks(width)
        .map(|line| String::from_iter(line.iter()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::{execute_fold, FoldInstruction};

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn parse() {
        let (set, instructions) = crate::parse(TEST_INPUT);
        assert_eq!(18, set.len());
        assert!(set.contains(&(6, 12)));
        assert_eq!(2, instructions.len());
        assert_eq!(FoldInstruction::Y(7), instructions[0]);
    }

    #[test]
    fn part1() {
        let (set, instructions) = crate::parse(TEST_INPUT);

        assert_eq!(17, execute_fold(&set, &instructions[0]).len());
    }

    #[test]
    fn as_text() {
        let (mut set, instructions) = crate::parse(TEST_INPUT);
        for instruction in instructions {
            set = execute_fold(&set, &instruction);
        }

        assert_eq!(
            "#####
#...#
#...#
#...#
#####",
            crate::as_text(&set)
        );
    }
}
