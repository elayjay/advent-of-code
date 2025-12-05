//! Problem 4: https://adventofcode.com/2025/day/4

use indoc::formatdoc;

const PAPER: char = '@';
const EMPTY: char = '.';

struct Grid<T> {
    width: usize,
    data: Vec<T>,
}

impl Grid<char> {
    fn from_string(s: &str) -> Result<Self, String> {
        let mut lines = s.lines();
        let first = lines
            .next()
            .ok_or("Empty string given to Grid::from_string")?;
        let width = first.len();

        width
            .checked_mul(width)
            .ok_or_else(|| formatdoc! {
                "First line of string is {} chars wide. A grid of this size is larger than the largest possible machine word.",
                width
            })?;

        let data: Vec<char> = s.lines().flat_map(|line| line.chars()).collect();

        if data.len() != width * width {
            return Err(format!(
                "Input contains {} characters but expected {width}x{width} = {}",
                data.len(),
                width * width
            ));
        }

        Ok(Grid { width, data })
    }

    /// Returns `None` if (`x`,`y`) is out of bounds
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if self.is_in_bounds(x, y) {
            return Some(self.data[y * self.width + x]);
        }
        None
    }

    fn set(&mut self, x: usize, y: usize, value: char) -> Result<(), String> {
        if self.is_in_bounds(x, y) {
            self.data[y * self.width + x] = value;
            return Ok(());
        }
        Err(format!(
            "Position ({x},{y}) out of bounds for grid of size ({},{}).",
            self.width, self.width
        ))
    }

    fn neighboring_objects(&self, x: usize, y: usize, object: char) -> usize {
        let mut count = 0;

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x.checked_add_signed(dx);
                let ny = y.checked_add_signed(dy);
                let (nx, ny) = match (nx, ny) {
                    (Some(nx), Some(ny)) if self.is_in_bounds(nx, ny) => (nx, ny),
                    _ => continue, // out of bounds - assume no object exists there
                };

                if self.get(nx, ny) == Some(object) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_forklift_accessible(&self, x: usize, y: usize) -> bool {
        if self.get(x, y) != Some(PAPER) {
            return false;
        }
        self.neighboring_objects(x, y, PAPER) < 4
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.width
    }

    fn try_taking(&mut self, x: usize, y: usize) -> bool {
        if !self.is_forklift_accessible(x, y) || self.set(x, y, EMPTY).is_err() {
            return false;
        }
        true
    }
}

fn part_1(input: &str) -> Result<usize, String> {
    let grid = Grid::from_string(input)?;

    let mut object_count = 0;

    for x in 0..grid.width {
        for y in 0..grid.width {
            if grid.is_forklift_accessible(x, y) {
                object_count += 1;
            }
        }
    }

    Ok(object_count)
}

fn part_2(input: &str) -> Result<usize, String> {
    let mut grid = Grid::from_string(input)?;

    let mut curr: usize = 0;
    let mut prev: usize = curr;
    loop {
        for x in 0..grid.width {
            for y in 0..grid.width {
                if grid.try_taking(x, y) {
                    curr += 1;
                }
            }
        }

        if curr == prev {
            break;
        }
        prev = curr;
    }

    Ok(curr)
}

pub fn p4_1() {
    match part_1(super::input_to_string(4, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 04 Part 1: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 04 Part 1: ERROR: {}", e);
        }
    }
}

pub fn p4_2() {
    match part_2(super::input_to_string(4, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 04 Part 2: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 04 Part 2: ERROR: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part_1() {
        let result = super::part_1(EXAMPLE);
        assert_eq!(result, Ok(13));
    }

    #[test]
    fn part_2() {
        let result = super::part_2(EXAMPLE);
        assert_eq!(result, Ok(43));
    }
}
