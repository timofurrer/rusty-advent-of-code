use std::collections::HashMap;

// include my puzzle input into binary
static PUZZLE_INPUT: &'static str = include_str!("input.txt");

fn main() {
    let solution_part_1 = solve_part_1(PUZZLE_INPUT);
    println!("Part 1 solution is '{:?}'", solution_part_1);
    let solution_part_2 = solve_part_2(PUZZLE_INPUT);
    println!("Part 2 solution is '{:?}'", solution_part_2);
}

fn solve_part_1(input: &str) -> Result<i32, ()> {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut letter_frequencies = HashMap::new();
        for c in line.chars() {
            *letter_frequencies.entry(c).or_insert(0) += 1;
        }

        if letter_frequencies.iter().any(|(_k, &v)| v == 2) {
            twos += 1;
        }
        if letter_frequencies.iter().any(|(_k, &v)| v == 3) {
            threes += 1;
        }
    }

    Ok(twos * threes)
}

fn solve_part_2(input: &str) -> Result<String, &str> {
    for current_box_id in input.lines() {
        for other_box_id in input.lines() {
            if current_box_id == other_box_id {
                continue;  // we are comparing against ourselves
            }

            let mut common_letters = String::new();
            for (a, b) in current_box_id.chars().zip(other_box_id.chars()) {
                if a == b {
                    common_letters.push(a);
                }
            }

            // check if we've found the similar box
            if common_letters.len() == current_box_id.len() - 1 {
                return Ok(common_letters);
            }
        }
    }

    Err("Could not find the correct box")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // given
        let expected_solution = 4980;

        // when
        let actual_solution = solve_part_1(PUZZLE_INPUT).unwrap();

        // then
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn test_part_2() {
        // given
        let expected_solution = "qysdtrkloagnfozuwujmhrbvx";

        // when
        let actual_solution = solve_part_2(PUZZLE_INPUT).unwrap();

        // then
        assert_eq!(actual_solution, expected_solution);
    }
}
