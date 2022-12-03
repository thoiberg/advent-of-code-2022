use std::cmp::Ordering;

fn main() {
    let data = process_input(read_data());

    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);

    let part_two_data = process_input_for_part_two(read_data());
    let part_two_answer: u32 = part_two_solution(&part_two_data);

    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(rounds: &Vec<FirstRound>) -> u32 {
    rounds.into_iter().fold(0, |acc, round| acc + round.play())
}

fn part_two_solution(rounds: &Vec<SecondRound>) -> u32 {
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

#[derive(Clone, Copy)]
struct SecondRound {
    desired_outcome: Outcome,
    opponent_hand: Hand,
}

impl SecondRound {
    pub fn new(player_hand: char, opponent_hand: char) -> Self {
        let desired_outcome = match player_hand {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!(),
        };
        let opponent_move = match opponent_hand {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!(),
        };

        Self {
            desired_outcome: desired_outcome,
            opponent_hand: opponent_move,
        }
    }

    pub fn play(self: Self) -> u32 {
        let outcome_points = match self.desired_outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        let type_points = match self.desired_outcome {
            Outcome::Lose => match self.opponent_hand {
                Hand::Rock => 3, // need scissors
                Hand::Paper => 1,
                Hand::Scissors => 2,
            },
            Outcome::Draw => match self.opponent_hand {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            },
            Outcome::Win => match self.opponent_hand {
                Hand::Rock => 2,
                Hand::Paper => 3,
                Hand::Scissors => 1,
            },
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

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
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

fn process_input_for_part_two(data: &str) -> Vec<SecondRound> {
    data.split("\n")
        .into_iter()
        .map(|round| {
            let moves: Vec<&str> = round.split(" ").collect();

            let boop: Vec<char> = moves[0].chars().collect();
            let troop: Vec<char> = moves[1].chars().collect();

            SecondRound::new(troop[0], boop[0])
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

    #[test]
    fn test_part_two_example() {
        let test_data = process_input_for_part_two("A Y\nB X\nC Z");
        let answer = part_two_solution(&test_data);

        assert_eq!(answer, 12)
    }
}
