fn main() {
    println!(
        "{}",
        lowest_alignment_cost(parse_input(include_str!("input.txt").trim()))
    );
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|str| str.parse::<i32>().unwrap())
        .collect()
}

fn lowest_alignment_cost(mut crab_positions: Vec<i32>) -> i32 {
    crab_positions.sort();

    let mean = crab_positions[crab_positions.len() / 2];
    let average_position: i32 =
        (crab_positions.iter().sum::<i32>() as f32 / crab_positions.len() as f32).round() as i32;
    dbg!(average_position, mean);

    crab_positions.iter().map(|pos| (pos - mean).abs()).sum()
}

#[test]
fn test() {
    assert_eq!(
        37,
        lowest_alignment_cost(parse_input("16,1,2,0,4,2,7,1,2,14"))
    )
}

// 0 1 1 2 2 2 4 7 14 16
