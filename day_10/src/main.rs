use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while1};
use nom::combinator::{all_consuming, map};
use nom::sequence::preceded;
use nom::{Finish, IResult};

fn main() {
    let example_data = "noop\naddx 3\naddx -5";
    let data = process_data(example_data);

    // for datum in data {
    //     println!("{:?}", datum);
    // }

    part_one_solution(&data);
}

fn part_one_solution(commands: &Vec<Command>) -> u32 {
    let mut value = 1;
    let mut cycle_count = 0;

    for command in commands {
        println!("running!");
        println!("command: {:?}", &command);
        match command {
            Command::Noop(_) => {
                cycle_count += 1;
            }
            Command::Add(add) => {
                println!("value: {}", add.value);
                cycle_count += 2;
                value += add.value
            }
        }
    }

    println!("cycle count: {}", cycle_count);
    println!("value: {}", value);
    cycle_count
    // iterate through
    //  if noop, increase clock count and
    // naive: check the command two before and execute on the mut value

    // unimplemented!()
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
        (|x: &str| x.parse::<i32>().unwrap()),
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
