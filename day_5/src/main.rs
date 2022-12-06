use std::{
    collections::{BTreeMap, VecDeque},
    str::FromStr,
};

fn main() {
    let mut data = process_data(read_data());

    let answer = part_one_solution(&mut data.0, &mut data.1);
    println!("The Answer to Part One is: {}", answer);
}

fn part_one_solution(stacks: &mut [Stack], moves: &mut [Move]) -> String {
    for box_move in moves {
        let stack_to_move_from = &mut stacks[(box_move.from - 1) as usize];

        let mut boxes_to_move = stack_to_move_from
            .boxes
            .split_off(stack_to_move_from.boxes.len() - (box_move.amount as usize));

        boxes_to_move.reverse();

        let stack_to_move_to = &mut stacks[(box_move.to - 1) as usize];
        stack_to_move_to.boxes.append(&mut boxes_to_move);
    }

    let top_boxes: Vec<String> = stacks
        .iter()
        .map(|stack| String::from(stack.boxes.last().unwrap().to_owned()))
        .collect();

    top_boxes.join("")
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_data(data: &str) -> (Vec<Stack>, Vec<Move>) {
    // TODO: collect into tuple
    let stuff: Vec<&str> = data.split("\n\n").collect();

    let stacks = build_stacks(stuff[0]);
    let moves = build_moves(stuff[1]);

    (stacks, moves)
}

fn build_stacks(stacks: &str) -> Vec<Stack> {
    let mut stack: Vec<&str> = stacks.lines().collect();

    stack.reverse();

    let mut deque_stack: VecDeque<&str> = stack.into_iter().collect();
    let mut char_indices: BTreeMap<usize, Vec<char>> = BTreeMap::new();

    for (idx, character) in deque_stack[0].chars().enumerate() {
        if character.is_numeric() {
            char_indices.insert(idx, Vec::new());
        }
    }

    deque_stack.pop_front();

    for line in deque_stack {
        let line_chars: Vec<char> = line.chars().collect();

        char_indices.iter_mut().for_each(|(indx, boxes)| {
            let box_value = line_chars.get(*indx);
            if let Some(value) = box_value {
                if value.is_alphabetic() {
                    boxes.push(*value);
                }
            }
        });
    }

    char_indices
        .into_values()
        .map(|boxes| Stack { boxes })
        .collect()
}

fn build_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .into_iter()
        .map(|move_line| {
            // TODO: don't just unwrap, handle better
            Move::from_str(move_line).unwrap()
        })
        .collect()
}

#[derive(Debug)]
struct Stack {
    boxes: Vec<char>,
}
#[derive(Debug)]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

impl FromStr for Move {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');

        let (Some("move"), Some(amount), Some("from"), Some(from_stack), Some("to"), Some(to_stack), None) = (words.next(), words.next(),words.next(),words.next(), words.next(), words.next(), words.next()) else {
            return Err(color_eyre::eyre::eyre!("expected move <from_stack> to <to_stack> EOF, got {s:?}"));
        };

        Ok(Move {
            // TODO: See if I can combine the parsing with the let statement
            from: from_stack.parse::<u32>().unwrap(),
            to: to_stack.parse::<u32>().unwrap(),
            amount: amount.parse::<u32>().unwrap(),
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
        let mut data = process_data(&test_data());
        let answer = part_one_solution(&mut data.0, &mut data.1);

        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_part_one_solution() {
        let mut data = process_data(&read_data());
        let answer = part_one_solution(&mut data.0, &mut data.1);

        assert_eq!(answer, "QGTHFZBHV");
    }
}
