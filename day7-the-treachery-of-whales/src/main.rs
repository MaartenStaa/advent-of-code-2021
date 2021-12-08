fn main() {
    let input = parse_input(include_str!("input.txt").trim());

    println!("Part 1: {}", lowest_alignment_cost_mean(input.clone()));
    println!("Part 2: {}", lowest_alignment_cost_range(input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|str| str.parse::<i32>().unwrap())
        .collect()
}

fn lowest_alignment_cost_mean(mut crab_positions: Vec<i32>) -> i32 {
    crab_positions.sort_unstable();

    let mean = crab_positions[crab_positions.len() / 2];

    crab_positions.iter().map(|pos| (pos - mean).abs()).sum()
}

fn lowest_alignment_cost_range(crab_positions: Vec<i32>) -> i32 {
    let max = crab_positions
        .iter()
        .max()
        .expect("The crab positions vec should not be empty");

    (0..=*max)
        .map(|target| {
            crab_positions
                .iter()
                .flat_map(|pos| 1..=((pos - target).abs()))
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(
        37,
        lowest_alignment_cost_mean(parse_input("16,1,2,0,4,2,7,1,2,14"))
    )
}

#[test]
fn test_part2() {
    assert_eq!(
        168,
        lowest_alignment_cost_range(parse_input("16,1,2,0,4,2,7,1,2,14"))
    )
}
