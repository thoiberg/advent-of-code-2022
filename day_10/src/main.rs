use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{all_consuming, map};
use nom::sequence::preceded;
use nom::{Finish, IResult};

fn main() {
    let data = process_data(read_data());

    let part_one_answer = part_one_solution(&data);
    println!("The solution for Part One is: {}", part_one_answer);
}

fn part_one_solution(commands: &Vec<Command>) -> i32 {
    let mut value = 1;
    let mut cycle_count = 0;
    let mut signals: Vec<i32> = vec![];

    for command in commands {
        let cycle_step: i32;
        let mut value_increase = 0;

        match command {
            Command::Noop(_) => {
                cycle_step = 1;
            }
            Command::Add(add) => {
                cycle_step = 2;
                value_increase = add.value;
            }
        }

        for cycle_step in cycle_count + 1..=cycle_count + cycle_step {
            if [20, 60, 100, 140, 180, 220].contains(&cycle_step) {
                signals.push(cycle_step * value);
            }

            cycle_count += 1;
        }

        value += value_increase
    }

    signals.into_iter().sum()
}

fn process_data(data: &str) -> Vec<Command> {
    data.lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
        .collect()
}

fn parse_line(line: &str) -> IResult<&str, Command> {
    alt((
        map(process_noop, Command::Noop),
        map(process_add, Command::Add),
    ))(line)
}

fn process_noop(line: &str) -> IResult<&str, Noop> {
    map(tag("noop"), |_| Noop)(line)
}

fn process_num(line: &str) -> IResult<&str, i32> {
    map(
        take_while1(|c: char| "-1234567890".contains(c)),
        |x: &str| x.parse::<i32>().unwrap(),
    )(line)
}

fn process_add(line: &str) -> IResult<&str, Add> {
    map(preceded(tag("addx "), process_num), |x: i32| Add {
        value: x,
    })(line)
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

#[derive(Debug)]
struct Add {
    value: i32,
}
#[derive(Debug)]
struct Noop;

#[derive(Debug)]
enum Command {
    Add(Add),
    Noop(Noop),
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = process_data(include_str!("../data/example_data"));
        let answer = part_one_solution(&data);

        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_data(read_data());
        let answer = part_one_solution(&data);

        assert_eq!(answer, 14760);
    }
}
