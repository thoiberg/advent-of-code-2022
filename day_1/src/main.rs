fn main() {
    let data = puzzle_data();

    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);

    let part_two_answer = part_two_solution(&data);

    println!("Part Two Solution is: {}", part_two_answer);
}

fn part_one_solution(data: &Vec<Vec<u32>>) -> u32 {
    data.into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .unwrap()
}

fn part_two_solution(data: &Vec<Vec<u32>>) -> u32 {
    let mut calorie_counts: Vec<u32> = data.into_iter().map(|elf| elf.into_iter().sum()).collect();

    calorie_counts.sort_by(|a, b| a.cmp(b).reverse());

    calorie_counts[0..=2].into_iter().sum()
}

fn puzzle_data() -> Vec<Vec<u32>> {
    let data = read_data();

    return data
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<Vec<u32>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn test_part_one_example() {
        let answer = part_one_solution(&test_data());

        assert_eq!(answer, 24000)
    }

    #[test]
    fn test_part_one_solution() {
        let answer = part_one_solution(&puzzle_data());

        assert_eq!(answer, 71471);
    }

    #[test]
    fn test_part_two_example() {
        let answer = part_two_solution(&test_data());

        assert_eq!(answer, 45000)
    }

    #[test]
    fn test_part_two_solution() {
        let answer = part_two_solution(&puzzle_data());

        assert_eq!(answer, 211189);
    }
}
