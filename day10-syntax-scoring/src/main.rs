use std::{iter::Peekable, str::Chars};

fn main() {
    println!(
        "{}",
        first_illegal_character_score_sum(include_str!("input.txt"))
    );
}

fn first_illegal_character_score_sum(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .filter_map(|chunk| first_illegal_character(&chunk))
        .map(illegal_character_score)
        .sum()
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

fn first_illegal_character(chunk: &Chunk) -> Option<char> {
    if let Some(illegal_child_chunk_closing_brace) =
        chunk.children.iter().find_map(first_illegal_character)
    {
        return Some(illegal_child_chunk_closing_brace);
    }

    match chunk.closing_brace {
        ClosingBrace::Invalid(c) => Some(c),
        _ => None,
    }
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

fn parse(input: &str) -> Chunk {
    parse_internal(&mut input.chars().peekable())
}

fn parse_internal(input: &mut Peekable<Chars>) -> Chunk {
    // dbg!("parse_internal", &input);
    let opening_brace = match input.peek() {
        Some('(') => Brace::Parentheses,
        Some('[') => Brace::SquareBrackets,
        Some('{') => Brace::CurlyBraces,
        Some('<') => Brace::AngleBrackets,
        _ => panic!("Invalid opening brace: {}", input.peek().unwrap()),
    };
    // dbg!(&opening_brace);

    // Consume opening brace
    input.next();

    let mut children = vec![];
    let mut closing_brace = ClosingBrace::Missing;
    while let Some(c) = input.peek() {
        // dbg!(c);
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

#[test]
fn test_parse() {
    assert_eq!(
        Chunk {
            opening_brace: Brace::Parentheses,
            children: vec![],
            closing_brace: ClosingBrace::Correct
        },
        parse("()")
    );
    assert_eq!(
        Chunk {
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
        },
        parse("[([]))")
    );
    assert_eq!(
        Chunk {
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
        },
        parse("{[](<")
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
    assert_eq!(
        26397,
        first_illegal_character_score_sum(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        )
    )
}
