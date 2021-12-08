use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        count_occurences(parse_input(include_str!("input.txt")), &[1, 4, 7, 8])
    )
}

struct Entry<'a> {
    signal_patterns: Vec<&'a str>,
    output_values: Vec<&'a str>,
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");

            Entry {
                signal_patterns: parts.next().unwrap().split_ascii_whitespace().collect(),
                output_values: parts.next().unwrap().split_ascii_whitespace().collect(),
            }
        })
        .collect()
}

fn count_occurences(input: Vec<Entry>, search_for: &[u8]) -> usize {
    let search_for: HashSet<_> = search_for
        .into_iter()
        .map(|num| match num {
            0 => 6,
            1 => 2,
            2 => 5,
            3 => 5,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 3,
            8 => 7,
            9 => 6,
            _ => panic!("Unexpected search term"),
        })
        .collect();

    input
        .iter()
        .map(|entry| {
            entry
                .output_values
                .iter()
                .filter(|value| search_for.contains(&value.len()))
                .count()
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[test]
fn test() {
    assert_eq!(26, count_occurences(parse_input(TEST_INPUT), &[1, 4, 7, 8]))
}
