fn main() {
    let data = puzzle_data();

    let part_one_answer = part_one_solution(&data);

    println!("Part One Solution is: {}", part_one_answer);
}

fn part_one_solution(data: &Vec<Vec<u32>>) -> u32 {
    let mut max = 0;

    for elf in data {
        let total_calories = elf.into_iter().sum();
        max = if total_calories > max {
            total_calories
        } else {
            max
        }
    }

    max
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

    #[test]
    fn test_part_one_example() {
        let data: Vec<Vec<u32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let answer = part_one_solution(&data);

        assert_eq!(answer, 24000)
    }

    #[test]
    fn test_part_one_solution() {
        let answer = part_one_solution(&puzzle_data());

        assert_eq!(answer, 71471);
    }
}
