fn main() {
    let mut players = vec![Player::new(1), Player::new(3)];
    let mut die = DeterministicDie::new();

    let winner = simulate_until_winner(&mut players, &mut die);
    let loser = find_losing_player(&players, winner).expect("Expected to find the loser");

    println!("{}", loser.score * die.rolls);
}

fn find_losing_player(players: &[Player], winner: usize) -> Option<&Player> {
    players
        .iter()
        .enumerate()
        .filter_map(|(index, player)| if index != winner { Some(player) } else { None })
        .next()
}

fn simulate_until_winner(players: &mut [Player], die: &mut DeterministicDie) -> usize {
    'outer: loop {
        for (index, player) in players.iter_mut().enumerate() {
            player.move_pawn(die.roll() + die.roll() + die.roll());

            if player.score >= 1000 {
                break 'outer index;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::{find_losing_player, simulate_until_winner, DeterministicDie, Player};

    #[test]
    fn part1() {
        let mut players = vec![Player::new(4), Player::new(8)];
        let mut die = DeterministicDie::new();

        let winner = simulate_until_winner(&mut players, &mut die);
        assert_eq!(0, winner);

        let losing_player = find_losing_player(&players, winner).unwrap();
        assert_eq!(739785, losing_player.score * die.rolls);
    }
}
