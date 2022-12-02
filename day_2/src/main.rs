use std::{cmp::Ordering, fs::read_to_string};

fn main() {
    let data = process_input(read_data());

    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);
}

#[derive(Clone, Copy)]
struct Round {
    player_hand: Hand,
    opponent_hand: Hand,
}

impl Round {
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
        match self.player_hand {
            Hand::Rock => {
                1 + match self.opponent_hand {
                    Hand::Rock => 3,
                    Hand::Scissors => 6,
                    Hand::Paper => 0,
                }
            }
            Hand::Paper => {
                2 + match self.opponent_hand {
                    Hand::Rock => 6,
                    Hand::Scissors => 0,
                    Hand::Paper => 3,
                }
            }
            Hand::Scissors => {
                3 + match self.opponent_hand {
                    Hand::Rock => 0,
                    Hand::Scissors => 3,
                    Hand::Paper => 6,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn part_one_solution(rounds: &Vec<Round>) -> u32 {
    rounds.into_iter().fold(0, |acc, round| acc + round.play())
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_input(data: &str) -> Vec<Round> {
    data.split("\n")
        .into_iter()
        .map(|round| {
            let moves: Vec<&str> = round.split(" ").collect();

            let boop: Vec<char> = moves[0].chars().collect();
            let troop: Vec<char> = moves[1].chars().collect();

            Round::new(troop[0], boop[0])
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Round> {
        process_input("A Y\nB X\nC Z")
    }

    #[test]
    fn test_part_one_example() {
        let answer = part_one_solution(test_data());

        assert_eq!(answer, 15);
    }
}
