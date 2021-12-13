use std::{iter::Peekable, str::Chars};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", first_illegal_character_score_sum(input));
    println!("Part 2: {}", middle_autocomplete_score(input));
}

fn first_illegal_character_score_sum(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .filter_map(|chunk| first_illegal_character(&chunk))
        .map(illegal_character_score)
        .sum()
}

fn middle_autocomplete_score(input: &str) -> usize {
    let mut scores: Vec<_> = input
        .lines()
        .map(parse)
        .filter(|chunk| first_illegal_character(chunk).is_none())
        .filter_map(|chunk| get_autocomplete(&chunk))
        .map(|autocomplete| get_autocomplete_score(&autocomplete))
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[derive(Debug, PartialEq)]
enum Brace {
    Parentheses,
    SquareBrackets,
    CurlyBraces,
    AngleBrackets,
}

#[derive(Debug, PartialEq)]
enum ClosingBrace {
    Correct,
    Missing,
    Invalid(char),
}

#[derive(Debug, PartialEq)]
struct Chunk {
    opening_brace: Brace,
    children: Vec<Chunk>,
    closing_brace: ClosingBrace,
}

fn first_illegal_character(chunk: &[Chunk]) -> Option<char> {
    chunk.iter().find_map(|chunk| {
        if let Some(illegal_child_chunk_closing_brace) = first_illegal_character(&chunk.children) {
            return Some(illegal_child_chunk_closing_brace);
        }

        match chunk.closing_brace {
            ClosingBrace::Invalid(c) => Some(c),
            _ => None,
        }
    })
}

fn illegal_character_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn parse(input: &str) -> Vec<Chunk> {
    let mut chunks = vec![];
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        chunks.push(parse_internal(&mut chars));
    }

    chunks
}

fn parse_internal(input: &mut Peekable<Chars>) -> Chunk {
    let opening_brace = match input.peek() {
        Some('(') => Brace::Parentheses,
        Some('[') => Brace::SquareBrackets,
        Some('{') => Brace::CurlyBraces,
        Some('<') => Brace::AngleBrackets,
        _ => panic!("Invalid opening brace: {}", input.peek().unwrap()),
    };

    // Consume opening brace
    input.next();

    let mut children = vec![];
    let mut closing_brace = ClosingBrace::Missing;
    while let Some(c) = input.peek() {
        match c {
            ')' => {
                closing_brace = if opening_brace == Brace::Parentheses {
                    ClosingBrace::Correct
                } else {
                    ClosingBrace::Invalid(*c)
                };
                break;
            }
            ']' => {
                closing_brace = if opening_brace == Brace::SquareBrackets {
                    ClosingBrace::Correct
                } else {
                    ClosingBrace::Invalid(*c)
                };
                break;
            }
            '}' => {
                closing_brace = if opening_brace == Brace::CurlyBraces {
                    ClosingBrace::Correct
                } else {
                    ClosingBrace::Invalid(*c)
                };
                break;
            }
            '>' => {
                closing_brace = if opening_brace == Brace::AngleBrackets {
                    ClosingBrace::Correct
                } else {
                    ClosingBrace::Invalid(*c)
                };
                break;
            }
            _ => children.push(parse_internal(input)),
        }
    }

    // Consume the closing brace
    input.next();

    Chunk {
        opening_brace,
        children,
        closing_brace,
    }
}

fn get_autocomplete(chunk: &[Chunk]) -> Option<String> {
    // Note that only the last chunk could possibly have missing characters.
    // Otherwise it could not be the last chunk.
    match chunk.last() {
        Some(chunk) => {
            if chunk.closing_brace != ClosingBrace::Missing {
                // Then none of the children's closing braces could be missing either
                return None;
            }

            Some(format!(
                "{}{}",
                get_autocomplete(&chunk.children).unwrap_or_default(),
                match chunk.closing_brace {
                    ClosingBrace::Missing => match chunk.opening_brace {
                        Brace::Parentheses => ")",
                        Brace::SquareBrackets => "]",
                        Brace::CurlyBraces => "}",
                        Brace::AngleBrackets => ">",
                    },
                    _ => "",
                }
            ))
        }
        None => None,
    }
}

fn get_autocomplete_score(autocomplete: &str) -> usize {
    autocomplete.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![Chunk {
                opening_brace: Brace::Parentheses,
                children: vec![],
                closing_brace: ClosingBrace::Correct
            }],
            parse("()")
        );
        assert_eq!(
            vec![Chunk {
                opening_brace: Brace::SquareBrackets,
                children: vec![Chunk {
                    opening_brace: Brace::Parentheses,
                    children: vec![Chunk {
                        opening_brace: Brace::SquareBrackets,
                        children: vec![],
                        closing_brace: ClosingBrace::Correct
                    }],
                    closing_brace: ClosingBrace::Correct
                },],
                closing_brace: ClosingBrace::Invalid(')')
            }],
            parse("[([]))")
        );
        assert_eq!(
            vec![Chunk {
                opening_brace: Brace::CurlyBraces,
                children: vec![
                    Chunk {
                        opening_brace: Brace::SquareBrackets,
                        children: vec![],
                        closing_brace: ClosingBrace::Correct
                    },
                    Chunk {
                        opening_brace: Brace::Parentheses,
                        children: vec![Chunk {
                            opening_brace: Brace::AngleBrackets,
                            children: vec![],
                            closing_brace: ClosingBrace::Missing
                        },],
                        closing_brace: ClosingBrace::Missing
                    },
                ],
                closing_brace: ClosingBrace::Missing
            }],
            parse("{[](<")
        );
        assert_eq!(
            vec![
                Chunk {
                    opening_brace: Brace::Parentheses,
                    children: vec![],
                    closing_brace: ClosingBrace::Correct
                },
                Chunk {
                    opening_brace: Brace::SquareBrackets,
                    children: vec![Chunk {
                        opening_brace: Brace::CurlyBraces,
                        children: vec![],
                        closing_brace: ClosingBrace::Missing
                    },],
                    closing_brace: ClosingBrace::Missing
                },
            ],
            parse("()[{")
        );
    }

    #[test]
    fn test_first_illegal_character() {
        // {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
        assert_eq!(
            Some('}'),
            first_illegal_character(&parse("{([(<{}[<>[]}>{[]{[(<()>"))
        );
        // [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
        assert_eq!(
            Some(')'),
            first_illegal_character(&parse("[[<[([]))<([[{}[[()]]]"))
        );
        // [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
        assert_eq!(
            Some(']'),
            first_illegal_character(&parse("[{[{({}]{}}([{[{{{}}([]"))
        );
        // [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
        assert_eq!(
            Some(')'),
            first_illegal_character(&parse("[<(<(<(<{}))><([]([]()"))
        );
        // <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
        assert_eq!(
            Some('>'),
            first_illegal_character(&parse("<{([([[(<>()){}]>(<<{{"))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(26397, first_illegal_character_score_sum(TEST_INPUT))
    }

    #[test]
    fn test_get_autocomplete() {
        assert_eq!(
            Some(")}>]})".to_owned()),
            get_autocomplete(&parse("[(()[<>])]({[<{<<[]>>("))
        )
    }

    #[test]
    fn test_get_autocomplete_score() {
        assert_eq!(5566, get_autocomplete_score(")}>]})"))
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, middle_autocomplete_score(TEST_INPUT));
    }
}
