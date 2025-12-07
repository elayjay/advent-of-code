//! Problem 6: https://adventofcode.com/2025/day/6

use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|x| !x.is_empty()).collect();

        if lines.is_empty() {
            return Err(String::from("Empty file"));
        }

        let expected_cols = lines[0].len();
        let mut data = Vec::with_capacity(expected_cols * lines.len());
        for (idx, line) in lines.iter().enumerate() {
            if line.len() != expected_cols {
                return Err(format!(
                    "Non-uniform row length at line {idx}. Expected {expected_cols}, got {}",
                    line.len()
                ));
            }
            data.extend(line.chars());
        }

        Ok(Self {
            rows: lines.len(),
            cols: expected_cols,
            data,
        })
    }
}

impl Grid {
    /// A vector of characters representing a column at the specified index
    fn get_column(&self, col: usize) -> Vec<char> {
        let mut result = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            result.push(self.data[row * self.cols + col]);
        }
        result
    }

    /// Iterator over the columns of the grid from left to right
    fn iter_columns(&self) -> impl Iterator<Item = Vec<char>> + '_ {
        (0..self.cols).map(|col| self.get_column(col))
    }
}

fn calculate(op: char, args: &[String]) -> Result<u64, String> {
    let mut nums = Vec::<u64>::new();
    for arg in args {
        match arg.parse::<u64>() {
            Ok(num) => nums.push(num),
            Err(e) => return Err(format!("Unable to parse \"{arg}\" as a number: {e}.")),
        }
    }

    let val = match op {
        '*' => nums.iter().fold(1u64, |acc, &x| acc.saturating_mul(x)),
        '+' => nums.iter().sum(),
        ' ' => 0,
        unexpected => return Err(format!("Unexpected operation: {unexpected}.")),
    };
    Ok(val)
}

fn part_1(input: &str) -> Result<u64, String> {
    let grid = Grid::from_str(input)?;
    if grid.rows < 2 {
        return Err(String::from("Grid must have at least 2 rows."));
    }

    let mut operation = ' ';
    let mut result = 0u64;
    let mut args: Vec<String> = vec![String::new(); grid.rows - 1];

    for col in grid.iter_columns() {
        if col == vec![' '; col.len()] {
            // All numbers considered finished on a blank column - reset parsing state
            result += calculate(operation, &args)?;
            operation = ' ';
            args.iter_mut().for_each(|a| a.clear());
            continue;
        }

        // Build the numbers horizontally from N-1 rows
        for (idx, &digit_char) in col[..(col.len() - 1)].iter().enumerate() {
            if digit_char != ' ' {
                args[idx].push(digit_char);
            }
        }

        operation = match col.last() {
            None => {
                return Err(String::from(
                    "Empty column being processed, even though it should have been handled already.",
                ));
            }
            Some('*') => '*',
            Some('+') => '+',
            Some(' ') => operation,
            Some(unexpected) => {
                return Err(format!(
                    "Unexpected operation character \"{unexpected}\".\""
                ));
            }
        }
    }

    // Input may not end in a blank column, so perform the last calculation
    result += calculate(operation, &args)?;

    Ok(result)
}

fn part_2(input: &str) -> Result<u64, String> {
    let grid = Grid::from_str(input)?;
    if grid.rows < 2 {
        return Err(String::from("Grid must have at least 2 rows."));
    }

    let mut operation = ' ';
    let mut result = 0u64;
    let mut args = Vec::<String>::new();

    for col in grid.iter_columns() {
        if col == vec![' '; col.len()] {
            // All numbers considered finished on a blank column - reset parsing state
            result += calculate(operation, &args)?;
            operation = ' ';
            args.clear();
            continue;
        }

        // Build the numbers vertically from N-1 rows
        let chars_string = String::from_iter(col[..(col.len() - 1)].iter());
        let num = chars_string.trim();
        if num.is_empty() {
            return Err(String::from(
                "Empty column being processed, even though it should have been handled already.",
            ));
        } else {
            args.push(num.to_string());
        }

        operation = match col.last() {
            None => {
                return Err(String::from(
                    "Empty column being processed, even though it should have been handled already.",
                ));
            }
            Some('*') => '*',
            Some('+') => '+',
            Some(' ') => operation,
            Some(unexpected) => {
                return Err(format!(
                    "Unexpected operation character \"{unexpected}\".\""
                ));
            }
        }
    }

    // Input may not end in a blank column, so perform the last calculation
    result += calculate(operation, &args)?;

    Ok(result)
}

pub fn p6_1() {
    match part_1(super::input_to_string(6, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 06 Part 1: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 06 Part 1: ERROR: {}", e);
        }
    }
}

pub fn p6_2() {
    match part_2(super::input_to_string(6, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 06 Part 2: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 06 Part 2: ERROR: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use indoc::concatdoc;

    const EXAMPLE: &str = concatdoc!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    );

    #[test]
    fn grid_from_string() {
        let grid = EXAMPLE
            .parse::<Grid>()
            .map_err(|e| format!("Couldn't parse example: {e}"));
        let expected: Vec<char> = EXAMPLE.chars().filter(|c| *c != '\n').collect();
        match grid {
            Ok(grid) => {
                assert_eq!(grid.data, expected);
                assert_eq!(grid.rows, 4);
                assert_eq!(grid.cols, 15);
            }
            Err(e) => panic!("Failed to parse example: {e}"),
        }
    }

    #[test]
    fn iterate_columns() {
        // Handled in `grid_from_string`
        let columns = EXAMPLE
            .parse::<Grid>()
            .unwrap()
            .iter_columns()
            .collect::<Vec<_>>();
        let expected = [
            ['1', ' ', ' ', '*'],
            ['2', '4', ' ', ' '],
            ['3', '5', '6', ' '],
            [' ', ' ', ' ', ' '],
            ['3', '6', '9', '+'],
            ['2', '4', '8', ' '],
            ['8', ' ', ' ', ' '],
            [' ', ' ', ' ', ' '],
            [' ', '3', '2', '*'],
            ['5', '8', '1', ' '],
            ['1', '7', '5', ' '],
            [' ', ' ', ' ', ' '],
            ['6', '2', '3', '+'],
            ['4', '3', '1', ' '],
            [' ', ' ', '4', ' '],
        ];
        assert_eq!(columns, expected);
    }

    #[test]
    fn part_1() {
        // Handled in `grid_from_string`
        assert_eq!(super::part_1(EXAMPLE), Ok(4277556));
    }

    #[test]
    fn part_2() {
        // Handled in `grid_from_string`
        assert_eq!(super::part_2(EXAMPLE), Ok(3263827));
    }
}
