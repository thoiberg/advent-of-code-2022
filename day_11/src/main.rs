fn main() {
    println!("Hello, world!");
    let data = real_monkeys();
    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);
}

fn part_one_solution(data: &Vec<Monkey>) -> u32 {
    let mut monkeys = data.clone();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mc;

            // trick to get around multiple mut references for the same vec taken from
            // https://fasterthanli.me/series/advent-of-code-2022/part-11
            {
                let monkey = &mut monkeys[i];
                mc = monkey.clone();
                monkey.check_count += mc.items.len() as u32;
            }

            for item in mc.items.iter() {
                let after_inspection = (mc.operation)(item);
                let after_boredom = after_inspection / 3;

                let next_monkey = if (after_boredom % mc.test_amount) == 0 {
                    (mc.test)(Condition::True)
                } else {
                    (mc.test)(Condition::False)
                };

                monkeys[next_monkey].items.push(after_boredom);
            }

            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by_key(|monkey| std::cmp::Reverse(monkey.check_count));

    monkeys[0].check_count * monkeys[1].check_count
}

fn test_monkeys() -> Vec<Monkey> {
    let monkey0 = Monkey {
        items: vec![79, 98],
        operation: (|x| x * 19),
        test_amount: 23,
        test: |boop| match boop {
            Condition::True => 2,
            Condition::False => 3,
        },
        check_count: 0,
    };

    let monkey1 = Monkey {
        items: vec![54, 65, 75, 74],
        operation: |x| x + 6,
        test_amount: 19,
        test: |boop| match boop {
            Condition::True => 2,
            Condition::False => 0,
        },
        check_count: 0,
    };

    let monkey2 = Monkey {
        items: vec![79, 60, 97],
        operation: |x| x * x,
        test_amount: 13,
        test: |boop| match boop {
            Condition::True => 1,
            Condition::False => 3,
        },
        check_count: 0,
    };
    let monkey3 = Monkey {
        items: vec![74],
        operation: |x| x + 3,
        test_amount: 17,
        test: |boop| match boop {
            Condition::True => 0,
            Condition::False => 1,
        },
        check_count: 0,
    };

    vec![monkey0, monkey1, monkey2, monkey3]
}

fn real_monkeys() -> Vec<Monkey> {
    let monkey0 = Monkey {
        items: vec![54, 61, 97, 63, 74],
        operation: |x| x * 7,
        test_amount: 17,
        test: |condition| match condition {
            Condition::True => 5,
            Condition::False => 3,
        },
        check_count: 0,
    };
    let monkey1 = Monkey {
        items: vec![61, 70, 97, 64, 99, 83, 52, 87],
        operation: |x| x + 8,
        test_amount: 2,
        test: |condition| match condition {
            Condition::True => 7,
            Condition::False => 6,
        },
        check_count: 0,
    };
    let monkey2 = Monkey {
        items: vec![60, 67, 80, 65],
        operation: |x| x * 13,
        test_amount: 5,
        test: |condition| match condition {
            Condition::True => 1,
            Condition::False => 6,
        },
        check_count: 0,
    };
    let monkey3 = Monkey {
        items: vec![61, 70, 76, 69, 82, 56],
        operation: |x| x + 7,
        test_amount: 3,
        test: |condition| match condition {
            Condition::True => 5,
            Condition::False => 2,
        },
        check_count: 0,
    };
    let monkey4 = Monkey {
        items: vec![79, 98],
        operation: |x| x + 2,
        test_amount: 7,
        test: |condition| match condition {
            Condition::True => 0,
            Condition::False => 3,
        },
        check_count: 0,
    };
    let monkey5 = Monkey {
        items: vec![72, 79, 55],
        operation: |x| x + 1,
        test_amount: 13,
        test: |condition| match condition {
            Condition::True => 2,
            Condition::False => 1,
        },
        check_count: 0,
    };
    let monkey6 = Monkey {
        items: vec![63],
        operation: |x| x + 4,
        test_amount: 19,
        test: |condition| match condition {
            Condition::True => 7,
            Condition::False => 4,
        },
        check_count: 0,
    };
    let monkey7 = Monkey {
        items: vec![72, 51, 93, 63, 80, 86, 81],
        operation: |x| x * x,
        test_amount: 11,
        test: |condition| match condition {
            Condition::True => 0,
            Condition::False => 4,
        },
        check_count: 0,
    };

    vec![
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ]
}

fn process_data() {
    // split by \n\n
    // disregard Monkey x: (not needed)
    // grab the starting items
    // Grab the operation
    // Grab the test
}

fn process_starting_items(i: &str) {}

fn read_data() -> &'static str {
    // include_str!("../data/puzzle_data")
    include_str!("../data/example_data")
}
#[derive(Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: fn(&u32) -> u32,
    test_amount: u32,
    test: fn(Condition) -> usize,
    check_count: u32,
}

enum Condition {
    True,
    False,
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let monkeys = test_monkeys();
        let answer = part_one_solution(&monkeys);

        assert_eq!(answer, 10605);
    }

    #[test]
    fn test_part_one_solution() {
        let monkeys = real_monkeys();
        let answer = part_one_solution(&monkeys);

        assert_eq!(answer, 50172);
    }
}
