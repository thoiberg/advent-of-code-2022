use std::{collections::BTreeSet, str::FromStr};

fn main() {
    let data = process_data(read_data());

    let part_one_answer = part_one_solution(&data);
    println!("The solution for Part One is: {:?}", part_one_answer);

    let part_two_answer = part_two_solution(&data);
    println!("The solution for Part Two is: {:?}", part_two_answer);
}

fn part_one_solution(moves: &Vec<Move>) -> Option<usize> {
    let mut head = Marker::default();
    let mut tail = Marker::default();

    let mut tail_locations: BTreeSet<Coordinate> = BTreeSet::new();
    tail_locations.insert(tail.current_position);

    for head_move in moves {
        for _ in 1..=head_move.amount {
            let prev_coordinate = head.current_position;
            head.move_pos(&head_move.direction, 1);

            let horizontal_diff = (head.current_position.x - tail.current_position.x).abs();
            let vertical_diff = (head.current_position.y - tail.current_position.y).abs();

            if horizontal_diff > 1 || vertical_diff > 1 {
                tail.current_position = prev_coordinate;

                tail_locations.insert(tail.current_position);
            }
        }
    }

    Some(tail_locations.len())
}

fn part_two_solution(moves: &Vec<Move>) -> Option<usize> {
    let mut markers: Vec<Marker> = (1..=10).into_iter().map(|_| Marker::default()).collect();
    let mut tail_locations: BTreeSet<Coordinate> = BTreeSet::new();
    tail_locations.insert(markers[9].current_position);

    for head_move in moves {
        for _ in 1..=head_move.amount {
            let mut prev_marker = markers[0];

            for (idx, marker) in markers.iter_mut().enumerate() {
                if idx == 0 {
                    marker.move_pos(&head_move.direction, 1);
                } else {
                    let horizontal_diff =
                        prev_marker.current_position.x - marker.current_position.x;
                    let vertical_diff = prev_marker.current_position.y - marker.current_position.y;

                    if horizontal_diff.abs() > 1 || vertical_diff.abs() > 1 {
                        let steps_to_move_horizontally = match horizontal_diff.cmp(&0) {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => 1,
                        };
                        let steps_to_move_vertically = match vertical_diff.cmp(&0) {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => 1,
                        };

                        marker.current_position = Coordinate {
                            x: marker.current_position.x + steps_to_move_horizontally,
                            y: marker.current_position.y + steps_to_move_vertically,
                        };

                        if idx == 9 {
                            tail_locations.insert(marker.current_position);
                        }
                    }
                }

                prev_marker = *marker;
            }
        }
    }

    Some(tail_locations.len())
}

#[derive(Default, Clone, Copy, Debug)]
struct Marker {
    current_position: Coordinate,
}

impl Marker {
    pub fn move_pos(&mut self, direction: &Direction, step_amount: i32) {
        match direction {
            Direction::Up => self.move_up(step_amount),
            Direction::Left => self.move_left(step_amount),
            Direction::Down => self.move_down(step_amount),
            Direction::Right => self.move_right(step_amount),
        }
    }
    fn move_up(&mut self, step_amount: i32) {
        self.current_position = Coordinate {
            x: self.current_position.x,
            y: self.current_position.y + step_amount,
        };
    }

    fn move_left(&mut self, step_amount: i32) {
        self.current_position = Coordinate {
            x: self.current_position.x - step_amount,
            y: self.current_position.y,
        };
    }

    fn move_down(&mut self, step_amount: i32) {
        self.current_position = Coordinate {
            x: self.current_position.x,
            y: self.current_position.y - step_amount,
        };
    }

    fn move_right(&mut self, step_amount: i32) {
        self.current_position = Coordinate {
            x: self.current_position.x + step_amount,
            y: self.current_position.y,
        };
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(color_eyre::eyre::eyre!("Not a valid move: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn process_data(data: &str) -> Vec<Move> {
    data.lines()
        .map(|instruction| {
            let parts: Vec<&str> = instruction.split(' ').collect();

            Move {
                direction: Direction::from_str(parts[0]).unwrap(),
                amount: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let moves_str = include_str!("../data/example_data");
        let data = process_data(moves_str);

        let answer = part_one_solution(&data);

        assert_eq!(Some(13), answer);
    }

    #[test]
    fn test_part_one_solution() {
        let moves_str = include_str!("../data/puzzle_data");
        let data = process_data(moves_str);

        let answer = part_one_solution(&data);

        assert_eq!(Some(5907), answer);
    }

    #[test]
    fn test_part_two_example() {
        let moves_str = include_str!("../data/part_two_example_data");
        let data = process_data(moves_str);

        let answer = part_two_solution(&data);

        assert_eq!(Some(36), answer);
    }

    #[test]
    fn test_part_two_solution() {
        let data = process_data(include_str!("../data/puzzle_data"));

        let answer = part_two_solution(&data);

        assert_eq!(Some(2303), answer);
    }
}
