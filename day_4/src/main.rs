use std::collections::HashSet;

fn main() {
    let data = process_data(&read_data());

    let part_one_answer = part_one_solution(&data);
    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&data);
    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(elf_pairs: &Vec<Vec<HashSet<u32>>>) -> u32 {
    elf_pairs.into_iter().fold(0, |acc, elf_pair| {
        let left_side = &elf_pair[0];
        let right_side = &elf_pair[1];

        if left_side.is_subset(&right_side) || left_side.is_superset(&right_side) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_two_solution(elf_pairs: &Vec<Vec<HashSet<u32>>>) -> u32 {
    elf_pairs.into_iter().fold(0, |acc, elf_pair| {
        let left_side = &elf_pair[0];
        let right_side = &elf_pair[1];

        let overlapping_assignments: HashSet<&u32> = left_side.intersection(right_side).collect();

        if overlapping_assignments.len() >= 1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn process_data(data: &str) -> Vec<Vec<HashSet<u32>>> {
    data.split("\n")
        .into_iter()
        .map(|elf_pair| {
            elf_pair
                .split(",")
                .into_iter()
                .map(|assignment| {
                    let range_bookends: Vec<u32> = assignment
                        .split("-")
                        .into_iter()
                        .map(|range_bookend| range_bookend.parse::<u32>().unwrap())
                        .collect();

                    let start = range_bookends[0];
                    let end = range_bookends[1];

                    (start..=end).collect::<Vec<u32>>().into_iter().collect()
                })
                .collect()
        })
        .collect()
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Vec<HashSet<u32>>> {
        process_data(include_str!("../data/example_data"))
    }

    #[test]
    fn test_part_one_example() {
        let answer = part_one_solution(&test_data());

        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_data(include_str!("../data/puzzle_data"));
        let answer = part_one_solution(&data);

        assert_eq!(answer, 602);
    }

    #[test]
    fn test_part_two_example() {
        let answer = part_two_solution(&test_data());

        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part_two_solution() {
        let data = process_data(include_str!("../data/puzzle_data"));
        let answer = part_two_solution(&data);

        assert_eq!(answer, 891);
    }
}
