use std::{
    collections::{BTreeMap, VecDeque},
    str::FromStr,
};

fn main() {
    let data = process_data(read_data());

    let part_one_answer = part_one_solution(data.0.clone(), data.1.clone());
    println!("The Answer to Part One is: {}", part_one_answer);

    let part_two_answer = part_two_solution(data.0.clone(), data.1.clone());
    println!("The Answer to Part Two is: {}", part_two_answer);
}

fn part_one_solution(stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    move_boxes(stacks, moves, |boxes_to_move| {
        boxes_to_move.into_iter().rev().collect()
    })
}

fn part_two_solution(stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    move_boxes(stacks, moves, |boxes_to_move| boxes_to_move)
}

fn move_boxes(
    mut stacks: Vec<Stack>,
    moves: Vec<Move>,
    boxes_fn: fn(A: Vec<char>) -> Vec<char>,
) -> String {
    for box_move in moves {
        let stack_to_move_from = &mut stacks[(box_move.from - 1) as usize];

        let boxes_to_move = stack_to_move_from
            .boxes
            .split_off(stack_to_move_from.boxes.len() - (box_move.amount as usize));

        let mut boxes_to_append = boxes_fn(boxes_to_move);

        let stack_to_move_to = &mut stacks[(box_move.to - 1) as usize];
        stack_to_move_to.boxes.append(&mut boxes_to_append);
    }

    stacks
        .iter()
        .map(|stack| String::from(stack.boxes.last().unwrap().to_owned()))
        .fold(String::new(), |acc, top_box| format!("{}{}", acc, top_box))
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_data(data: &str) -> (Vec<Stack>, Vec<Move>) {
    let stuff: Vec<&str> = data.split("\n\n").collect();

    let stacks = build_stacks(stuff[0]);
    let moves = build_moves(stuff[1]);

    (stacks, moves)
}

fn build_stacks(stacks: &str) -> Vec<Stack> {
    let mut stack: VecDeque<_> = stacks.lines().collect();
    let mut char_indices: BTreeMap<usize, Vec<char>> = BTreeMap::new();

    let stack_names = stack.pop_back().unwrap();
    for (idx, character) in stack_names.chars().enumerate() {
        if character.is_numeric() {
            char_indices.insert(idx, Vec::new());
        }
    }

    stack.into_iter().rev().for_each(|line| {
        let line_chars: Vec<char> = line.chars().collect();

        char_indices.iter_mut().for_each(|(idx, boxes)| {
            let box_value = line_chars.get(*idx);
            if let Some(value) = box_value {
                if value.is_alphabetic() {
                    boxes.push(*value);
                }
            }
        });
    });

    char_indices
        .into_values()
        .map(|boxes| Stack { boxes })
        .collect()
}

fn build_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .into_iter()
        .map(|move_line| Move::from_str(move_line).unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Stack {
    boxes: Vec<char>,
}
#[derive(Debug, Clone, Copy)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');

        let (Some("move"), Some(amount), Some("from"), Some(from_stack), Some("to"), Some(to_stack), None) = (words.next(), words.next(),words.next(),words.next(), words.next(), words.next(), words.next()) else {
            return Err(color_eyre::eyre::eyre!("expected move <from_stack> to <to_stack> EOF, got {s:?}"));
        };

        Ok(Move {
            from: from_stack.parse::<usize>().unwrap(),
            to: to_stack.parse::<usize>().unwrap(),
            amount: amount.parse::<usize>().unwrap(),
        })
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> &'static str {
        include_str!("../data/example_data")
    }

    #[test]
    fn test_part_one_example() {
        let data = process_data(&test_data());
        let answer = part_one_solution(data.0, data.1);

        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_data(&read_data());
        let answer = part_one_solution(data.0, data.1);

        assert_eq!(answer, "QGTHFZBHV");
    }

    #[test]
    fn test_part_two_example() {
        let data = process_data(&test_data());
        let answer = part_two_solution(data.0, data.1);

        assert_eq!(answer, "MCD");
    }

    #[test]
    fn test_part_two_solution() {
        let data = process_data(&read_data());
        let answer = part_two_solution(data.0, data.1);

        assert_eq!(answer, "MGDMPSZTM");
    }
}
