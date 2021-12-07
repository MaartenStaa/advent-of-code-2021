fn main() {
    println!("{}", calculate_power_consumption(include_str!("input.txt")));
}

fn calculate_power_consumption(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let line_length = lines[0].len();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..line_length {
        let mut zeroes = 0;
        let mut ones = 0;

        for line in lines.iter() {
            let char = line[i];
            match char {
                b'0' => zeroes += 1,
                b'1' => ones += 1,
                _ => {}
            }
        }

        let gamma_bit = if zeroes > ones { 0 } else { 1 };
        let epsilon_bit = if zeroes < ones { 0 } else { 1 };

        gamma_rate = gamma_rate << 1 | gamma_bit;
        epsilon_rate = epsilon_rate << 1 | epsilon_bit;
    }

    gamma_rate * epsilon_rate
}

#[test]
fn test() {
    assert_eq!(
        198,
        calculate_power_consumption(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        )
    );
}
