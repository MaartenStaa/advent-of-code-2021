fn main() {
    let (numbers, boards) = parse_input(include_str!("input.txt"));
    let (winning_number, winner) = pick_winner(&numbers, boards);

    println!(
        "{:?}",
        winner.map(|winner| winner.get_score(winning_number))
    );
}

#[derive(Debug, Clone)]
struct Board {
    numbers: [u8; 25],
    picked: [bool; 25],
}

impl Board {
    fn new(numbers: &[u8]) -> Self {
        Self {
            numbers: numbers.try_into().expect("Incorrect board size"),
            picked: [false; 25],
        }
    }

    fn wins_after(&mut self, picked_num: u8) -> bool {
        for (index, num) in self.numbers.iter().enumerate() {
            if *num == picked_num {
                self.picked[index] = true;
            }
        }

        self.wins()
    }

    fn wins(&self) -> bool {
        [
            0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 0, 5, 10, 15, 20, 1, 6, 11, 16, 21, 2, 7, 12, 17, 22, 3, 8, 13, 18, 23, 4, 9,
            14, 19, 24,
        ]
        .chunks(5)
        .any(|window| window.iter().map(|index| self.picked[*index]).all(|p| p))
    }

    fn get_score(&self, winning_number: u8) -> usize {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(index, num)| {
                if !self.picked[index] {
                    Some(*num as usize)
                } else {
                    None
                }
            })
            .sum::<usize>()
            * (winning_number as usize)
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut numbers = Vec::new();
    let mut boards = Vec::new();

    let mut buffer = Vec::with_capacity(25);

    for line in input.lines() {
        if numbers.is_empty() {
            numbers.extend(line.split(',').filter_map(|str| str.parse::<u8>().ok()));

            continue;
        }

        if line.is_empty() {
            if !buffer.is_empty() {
                boards.push(Board::new(&buffer));
                buffer.clear();
            }

            continue;
        }

        buffer.extend(
            line.split_ascii_whitespace()
                .filter_map(|str| str.parse::<u8>().ok()),
        )
    }

    if !buffer.is_empty() {
        boards.push(Board::new(&buffer));
    }

    (numbers, boards)
}

fn pick_winner(numbers: &[u8], mut boards: Vec<Board>) -> (u8, Option<Board>) {
    for num in numbers {
        for board in boards.iter_mut() {
            if board.wins_after(*num) {
                return (*num, Some(board.clone()));
            }
        }
    }

    (0, None)
}

#[test]
fn test() {
    let (numbers, boards) = parse_input(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    );

    let (winning_number, winner) = pick_winner(&numbers, boards);
    assert!(winner.is_some());
    assert_eq!(4512, winner.unwrap().get_score(winning_number));
}
