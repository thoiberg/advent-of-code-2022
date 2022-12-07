use std::collections::HashSet;

fn main() {
    let data = read_data();

    let part_one_answer = part_one_solution(data);
    // TODO handle panicking in a nicer way
    println!("Part One Solution is: {}", part_one_answer.unwrap());

    let part_two_answer = part_two_solution(data);
    println!("Part Two Solution is: {}", part_two_answer.unwrap());
}

fn part_one_solution(signal: &str) -> Option<usize> {
    unique_subset(signal, 4)
}

fn part_two_solution(signal: &str) -> Option<usize> {
    unique_subset(signal, 14)
}

fn unique_subset(signal: &str, length: usize) -> Option<usize> {
    // taken from https://fasterthanli.me/series/advent-of-code-2022/part-6
    signal
        .as_bytes()
        .windows(length)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == length)
        .map(|pos| pos + length)
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

#[cfg(test)]
mod test_super {
    use super::*;

    const FIRST_EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SECOND_EXAMPLE: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const THIRD_EXAMPLE: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const FOURTH_EXAMPLE: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const FIFTH_EXAMPLE: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(FIRST_EXAMPLE), Some(7));
        assert_eq!(part_one_solution(SECOND_EXAMPLE), Some(5));
        assert_eq!(part_one_solution(THIRD_EXAMPLE), Some(6));
        assert_eq!(part_one_solution(FOURTH_EXAMPLE), Some(10));
        assert_eq!(part_one_solution(FIFTH_EXAMPLE), Some(11));
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(read_data()), Some(1965));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(FIRST_EXAMPLE), Some(19));
        assert_eq!(part_two_solution(SECOND_EXAMPLE), Some(23));
        assert_eq!(part_two_solution(THIRD_EXAMPLE), Some(23));
        assert_eq!(part_two_solution(FOURTH_EXAMPLE), Some(29));
        assert_eq!(part_two_solution(FIFTH_EXAMPLE), Some(26));
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(read_data()), Some(2773));
    }
}
