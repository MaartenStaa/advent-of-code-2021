fn main() {
    println!("{}", calculate_position_product(include_str!("input.txt")));
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

fn parse_instruction(line: &str) -> Option<Instruction> {
    match line.split_once(' ') {
        Some(("down", amount)) => Some(Instruction::Down(amount.parse().ok()?)),
        Some(("forward", amount)) => Some(Instruction::Forward(amount.parse().ok()?)),
        Some(("up", amount)) => Some(Instruction::Up(amount.parse().ok()?)),
        _ => None,
    }
}

#[test]
fn test() {
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
