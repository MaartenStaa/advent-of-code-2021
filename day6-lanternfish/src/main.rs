fn main() {
    let input = parse_input("4,2,4,1,5,1,2,2,4,1,1,2,2,2,4,4,1,2,1,1,4,1,2,1,2,2,2,2,5,2,2,3,1,4,4,4,1,2,3,4,4,5,4,3,5,1,2,5,1,1,5,5,1,4,4,5,1,3,1,4,5,5,5,4,1,2,3,4,2,1,2,1,2,2,1,5,5,1,1,1,1,5,2,2,2,4,2,4,2,4,2,1,2,1,2,4,2,4,1,3,5,5,2,4,4,2,2,2,2,3,3,2,1,1,1,1,4,3,2,5,4,3,5,3,1,5,5,2,4,1,1,2,1,3,5,1,5,3,1,3,1,4,5,1,1,3,2,1,1,1,5,2,1,2,4,2,3,3,2,3,5,1,5,1,2,1,5,2,4,1,2,4,4,1,5,1,1,5,2,2,5,5,3,1,2,2,1,1,4,1,5,4,5,5,2,2,1,1,2,5,4,3,2,2,5,4,2,5,4,4,2,3,1,1,1,5,5,4,5,3,2,5,3,4,5,1,4,1,1,3,4,4,1,1,5,1,4,1,2,1,4,1,1,3,1,5,2,5,1,5,2,5,2,5,4,1,1,4,4,2,3,1,5,2,5,1,5,2,1,1,1,2,1,1,1,4,4,5,4,4,1,4,2,2,2,5,3,2,4,4,5,5,1,1,1,1,3,1,2,1");

    println!("Part 1: {}", simulate_naive(input.clone(), 80).len());
    println!("Part 2: {}", simulate_optimised(&input, 256));
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect()
}

fn simulate_naive(mut fish: Vec<u8>, days: usize) -> Vec<u8> {
    for _ in 0..days {
        let mut new_fish = 0;

        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new_fish += 1;
            } else {
                *f -= 1;
            }
        }

        fish.extend(std::iter::repeat(8).take(new_fish));
    }

    fish
}

fn simulate_optimised(fish: &[u8], days: usize) -> usize {
    let mut counts = [0; 9];
    for f in fish {
        counts[*f as usize] += 1;
    }

    for _ in 0..days {
        counts = [
            // Shift all the counts down by 1
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            // 7 goes down to 6, and everything that was at 0 goes back to 6
            counts[0] + counts[7],
            counts[8],
            // Everything that was at 0 spawns a new 8
            counts[0],
        ];
    }

    counts.iter().sum()
}

#[test]
fn test_part1() {
    let input = parse_input("3,4,3,1,2");

    assert_eq!(5934, simulate_naive(input.clone(), 80).len());
    assert_eq!(5934, simulate_optimised(&input, 80));
}

#[test]
fn test_part2() {
    let input = parse_input("3,4,3,1,2");

    assert_eq!(26984457539, simulate_optimised(&input, 256));
}
