use std::collections::HashSet;

fn main() {
    let data = process_data(read_data());
    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);
}

fn part_one_solution(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks
        .into_iter()
        .fold(0, |acc, rucksack| acc + rucksack.total_value())
}

struct Rucksack {
    left_compartment: HashSet<char>,
    right_compartment: HashSet<char>,
}

impl Rucksack {
    pub fn shared_chars(&self) -> Vec<&char> {
        self.left_compartment
            .intersection(&self.right_compartment)
            .collect::<Vec<&char>>()
            .clone()
    }

    pub fn total_value(&self) -> u32 {
        self.shared_chars()
            .iter()
            .fold(0, |acc, shared_char| acc + self.value_for(shared_char))
    }

    fn value_for(&self, char: &char) -> u32 {
        let mut all_characters: Vec<char> = vec![];
        all_characters.append(&mut ('a'..='z').collect::<Vec<char>>());
        all_characters.append(&mut ('A'..='Z').collect::<Vec<char>>());

        match all_characters.iter().position(|&x| &x == char) {
            Some(idx) => (idx + 1).try_into().unwrap(),
            None => {
                panic!("Didn't find the character in either set, this shouldn't have happened...")
            }
        }
    }
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

fn process_data(data: &str) -> Vec<Rucksack> {
    data.split("\n")
        .into_iter()
        .map(|rucksack| {
            let chars: Vec<char> = rucksack.chars().collect();
            let halfway_point = chars.len() / 2;
            let left_compartment = chars[0..halfway_point].to_vec().into_iter().collect();
            let right_compartment = chars[halfway_point..].to_vec().into_iter().collect();

            Rucksack {
                left_compartment,
                right_compartment,
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Rucksack> {
        process_data(include_str!("../data/example_data"))
    }

    #[test]
    fn test_part_one_example() {
        let answer = part_one_solution(&test_data());

        assert_eq!(answer, 157);
    }

    #[test]
    fn test_part_one_solution() {
        let answer = part_one_solution(&process_data(read_data()));

        assert_eq!(answer, 8123);
    }
}
