use ndarray::{s, Array2};

fn main() {
    let trees = process_data(read_data());

    let part_one_solution = part_one_solution(trees);
    println!("Part One Answer is: {}", part_one_solution);
}

fn part_one_solution(trees: Array2<u32>) -> u32 {
    trees.indexed_iter().fold(0, |acc, ((row, col), tree)| {
        let above_visibility = trees.slice(s![0..row, col]).iter().all(|x| x < tree);
        let below_visibility = trees.slice(s![row + 1.., col]).iter().all(|x| x < tree);
        let left_visibility = trees.slice(s![row, 0..col]).iter().all(|x| x < tree);
        let right_visibility = trees.slice(s![row, col + 1..]).iter().all(|x| x < tree);

        if [
            above_visibility,
            below_visibility,
            left_visibility,
            right_visibility,
        ]
        .into_iter()
        .any(|x| x)
        {
            return acc + 1;
        } else {
            return acc;
        }
    })
}

fn read_data() -> &'static str {
    include_str!("../data/puzzle_data")
}

// TODO: Refactor allllllll of this (there should be an easier way to convert from a file to a 2DArray)
fn process_data(data: &str) -> Array2<u32> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in data.lines() {
        let line_grid = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        grid.push(line_grid);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let flat: Vec<u32> = grid.iter().flatten().cloned().collect();

    Array2::from_shape_vec((cols, rows), flat).unwrap()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = process_data(include_str!("../data/example_data"));

        let answer = part_one_solution(data);

        assert_eq!(answer, 21);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_data(read_data());

        let answer = part_one_solution(data);

        assert_eq!(answer, 1789);
    }
}
