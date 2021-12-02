fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", calculate_position_product(input));
    println!("Part 2: {}", calculate_position_product_part2(input));
}

enum Instruction {
    Down(isize),
    Forward(isize),
    Up(isize),
}

fn calculate_position_product(input: &str) -> isize {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for instruction in input.lines().filter_map(parse_instruction) {
        match instruction {
            Instruction::Down(amount) => depth += amount,
            Instruction::Forward(amount) => horizontal_position += amount,
            Instruction::Up(amount) => depth -= amount,
        }
    }

    horizontal_position * depth
}

fn calculate_position_product_part2(input: &str) -> isize {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in input.lines().filter_map(parse_instruction) {
        match instruction {
            Instruction::Down(amount) => aim += amount,
            Instruction::Forward(amount) => {
                horizontal_position += amount;
                depth += amount * aim;
            }
            Instruction::Up(amount) => aim -= amount,
        }
    }

    horizontal_position * depth
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    match line.split_once(' ') {
        Some(("down", amount)) => Some(Instruction::Down(amount.parse().ok()?)),
        Some(("forward", amount)) => Some(Instruction::Forward(amount.parse().ok()?)),
        Some(("up", amount)) => Some(Instruction::Up(amount.parse().ok()?)),
        _ => None,
    }
}

#[test]
fn test_part1() {
    assert_eq!(
        150,
        calculate_position_product(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        )
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        900,
        calculate_position_product_part2(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        )
    )
}
