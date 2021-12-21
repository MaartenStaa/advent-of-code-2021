use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

fn main() {
    let players_input = [Player::new(1), Player::new(3)];

    let mut players = players_input.clone();
    let mut die = DeterministicDie::new();

    let winner = simulate_deterministic(&mut players, &mut die, 1000);
    let loser = find_losing_player(&players, winner).expect("Expected to find the loser");

    println!("Part 1: {}", loser.score * die.rolls);

    let mut players = players_input;
    let outcomes = simulate_quantum(&mut players, &mut QuantumDie, 0, 21);

    println!("Part 2: {}", outcomes.iter().max().unwrap());
}

fn find_losing_player(players: &[Player], winner: usize) -> Option<&Player> {
    players
        .iter()
        .enumerate()
        .filter_map(|(index, player)| if index != winner { Some(player) } else { None })
        .next()
}

fn simulate_deterministic(
    players: &mut [Player; 2],
    die: &mut DeterministicDie,
    winning_score: u32,
) -> usize {
    'outer: loop {
        for (index, player) in players.iter_mut().enumerate() {
            player.move_pawn(die.roll() + die.roll() + die.roll());
            if player.score >= winning_score {
                break 'outer index;
            }
        }
    }
}

type QuantumLookupType = ([Player; 2], QuantumDie, usize, u32);

lazy_static! {
    static ref MEMOIZED: Mutex<HashMap<QuantumLookupType, [u64; 2]>> = Default::default();
}

fn simulate_quantum(
    players: &mut [Player; 2],
    die: &mut QuantumDie,
    current_turn: usize,
    winning_score: u32,
) -> [u64; 2] {
    let lookup_key = (players.clone(), die.clone(), current_turn, winning_score);
    if let Some(previously_calculated) = MEMOIZED.lock().unwrap().get(&lookup_key) {
        return *previously_calculated;
    }

    let mut wins = [0; 2];
    for (i, p) in players.iter().enumerate() {
        if p.score >= winning_score {
            wins[i] += 1;

            MEMOIZED.lock().unwrap().insert(lookup_key, wins);
            return wins;
        }
    }

    for outcome in die.three_roll_sums() {
        let mut cloned_players = players.clone();
        cloned_players[current_turn].move_pawn(outcome);

        // Explore each possible outcome
        for (player, wins_found) in simulate_quantum(
            &mut cloned_players,
            &mut die.clone(),
            (current_turn + 1) % players.len(),
            winning_score,
        )
        .iter()
        .enumerate()
        {
            wins[player] += wins_found;
        }
    }

    MEMOIZED.lock().unwrap().insert(lookup_key, wins);

    wins
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn new(position: u32) -> Self {
        Self { position, score: 0 }
    }

    fn move_pawn(&mut self, spaces: u32) {
        self.position = ((self.position - 1 + spaces) % 10) + 1;
        self.score += self.position;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct DeterministicDie {
    state: u32,
    rolls: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { state: 1, rolls: 0 }
    }

    fn roll(&mut self) -> u32 {
        let result = self.state;

        self.rolls += 1;
        self.state += 1;
        if self.state > 100 {
            self.state = 1;
        }

        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct QuantumDie;

impl QuantumDie {
    fn three_roll_sums(&mut self) -> [u32; 27] {
        [
            1 + 1 + 1,
            1 + 1 + 2,
            1 + 1 + 3,
            1 + 2 + 1,
            1 + 2 + 2,
            1 + 2 + 3,
            1 + 3 + 1,
            1 + 3 + 2,
            1 + 3 + 3,
            2 + 1 + 1,
            2 + 1 + 2,
            2 + 1 + 3,
            2 + 2 + 1,
            2 + 2 + 2,
            2 + 2 + 3,
            2 + 3 + 1,
            2 + 3 + 2,
            2 + 3 + 3,
            3 + 1 + 1,
            3 + 1 + 2,
            3 + 1 + 3,
            3 + 2 + 1,
            3 + 2 + 2,
            3 + 2 + 3,
            3 + 3 + 1,
            3 + 3 + 2,
            3 + 3 + 3,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        find_losing_player, simulate_deterministic, simulate_quantum, DeterministicDie, Player,
        QuantumDie,
    };

    #[test]
    fn part1() {
        let mut players = [Player::new(4), Player::new(8)];
        let mut die = DeterministicDie::new();

        let winner = simulate_deterministic(&mut players, &mut die, 1000);
        assert_eq!(0, winner);

        let losing_player = find_losing_player(&players, winner).unwrap();
        assert_eq!(739785, losing_player.score * die.rolls);
    }

    #[test]
    fn part2() {
        let mut players = [Player::new(4), Player::new(8)];

        let winner = simulate_quantum(&mut players, &mut QuantumDie, 0, 21);
        assert_eq!([444356092776315, 341960390180808], winner);
    }
}
