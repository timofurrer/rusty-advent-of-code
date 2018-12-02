use std::collections::HashSet;
use std::num::ParseIntError;

// include my puzzle input into binary
static PUZZLE_INPUT: &'static str = include_str!("input.txt");

fn main() {
    let solution_part_1 = solve_part_1(PUZZLE_INPUT);
    println!("Part 1 solution is '{:?}'", solution_part_1);
    let solution_part_2 = solve_part_2(PUZZLE_INPUT);
    println!("Part 2 solution is '{:?}'", solution_part_2);
}

fn solve_part_1(input: &str) -> Result<i32, ParseIntError> {
    let mut frequency = 0;
    for line in input.lines() {
        let freq_change: i32 = line.parse()?;
        frequency += freq_change;
    }
    Ok(frequency)
}

fn solve_part_2(input: &str) -> Result<i32, ParseIntError> {
    let mut frequency = 0;
    let mut previous_frequencies = HashSet::new();

    loop {
        for line in input.lines() {
            let freq_change: i32 = line.parse()?;
            if previous_frequencies.contains(&frequency) {
                return Ok(frequency);
            }
            previous_frequencies.insert(frequency);
            frequency += freq_change;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // given
        let expected_solution = 531;

        // when
        let actual_solution = solve_part_1(PUZZLE_INPUT).unwrap();

        // then
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn test_part_2() {
        // given
        let expected_solution = 76787;

        // when
        let actual_solution = solve_part_2(PUZZLE_INPUT).unwrap();

        // then
        assert_eq!(actual_solution, expected_solution);
    }
}
