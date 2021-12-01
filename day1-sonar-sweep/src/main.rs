fn main() {
    let readings = include_str!("input.txt")
        .lines()
        .filter_map(|str| str.parse::<u32>().ok())
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        readings.windows(2).filter(|pair| pair[1] > pair[0]).count()
    );

    println!(
        "Part 2: {}",
        readings
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|pair| pair[1] > pair[0])
            .count()
    );
}
