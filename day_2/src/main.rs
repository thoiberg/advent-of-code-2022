use std::cmp::Ordering;

fn main() {
    let data = process_input(read_data());

    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);
}

fn part_one_solution(rounds: &Vec<FirstRound>) -> u32 {
    rounds.into_iter().fold(0, |acc, round| acc + round.play())
}

#[derive(Clone, Copy)]
struct FirstRound {
    player_hand: Hand,
    opponent_hand: Hand,
}

impl FirstRound {
    pub fn new(player_hand: char, opponent_hand: char) -> Self {
        let player_move = match player_hand {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!(),
        };
        let opponent_move = match opponent_hand {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!(),
        };

        Self {
            player_hand: player_move,
            opponent_hand: opponent_move,
        }
    }

    pub fn play(self: Self) -> u32 {
        let outcome_points = match self.player_hand.cmp(&self.opponent_hand) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        let type_points = match self.player_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        outcome_points + type_points
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Ordering::Equal,
                Hand::Scissors => Ordering::Greater,
                Hand::Paper => Ordering::Less,
            },
            Hand::Paper => match other {
                Hand::Rock => Ordering::Greater,
                Hand::Scissors => Ordering::Less,
                Hand::Paper => Ordering::Equal,
            },
            Hand::Scissors => match other {
                Hand::Rock => Ordering::Less,
                Hand::Scissors => Ordering::Equal,
                Hand::Paper => Ordering::Greater,
            },
        }
    }
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_input(data: &str) -> Vec<FirstRound> {
    data.split("\n")
        .into_iter()
        .map(|round| {
            let moves: Vec<&str> = round.split(" ").collect();

            let boop: Vec<char> = moves[0].chars().collect();
            let troop: Vec<char> = moves[1].chars().collect();

            FirstRound::new(troop[0], boop[0])
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<FirstRound> {
        process_input("A Y\nB X\nC Z")
    }

    #[test]
    fn test_part_one_example() {
        let answer = part_one_solution(&test_data());

        assert_eq!(answer, 15);
    }

    #[test]
    fn test_part_one_solution() {
        let answer = part_one_solution(&process_input(read_data()));

        assert_eq!(answer, 17189)
    }
}
