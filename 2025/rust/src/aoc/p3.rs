fn ascii_char_to_digit(char: u8) -> Option<u8> {
    if char < b'0' || char > b'9' {
        return None;
    }
    Some(char - b'0')
}

struct Bank<const N: usize> {
    batteries: [u8; N],
}

impl<const N: usize> Bank<N> {
    fn from_string(s: &str) -> Result<Bank<N>, String> {
        let mut bank = Bank { batteries: [0; N] };
        let mut idx = 0usize;

        // I know the input only contains digits, so I don't need to parse as utf-8
        let bytes = s.as_bytes();
        if bytes.len() > bank.batteries.len() {
            Err(format!(
                "Expected {} characters, found {}",
                bank.batteries.len(),
                bytes.len(),
            ))?;
        }

        for byte in bytes {
            bank.batteries[idx] = ascii_char_to_digit(*byte).ok_or(format!(
                "Ascii value does not represent a digit: {} ({})",
                byte,
                char::from(*byte)
            ))?;
            idx += 1;
        }

        if idx < bank.batteries.len() {
            Err(format!(
                "Expected {} characters, found {}.",
                bank.batteries.len(),
                idx,
            ))?;
        }

        Ok(bank)
    }

    fn joltage(&self, num_batteries: usize) -> Result<u64, String> {
        if num_batteries > self.batteries.len() {
            return Err(format!(
                "Attempted to get joltage from {} batteries, but there are only {} in a pack.",
                num_batteries,
                self.batteries.len(),
            ));
        }

        let mut to_remove = self.batteries.len() - num_batteries;
        let mut stack: Vec<u8> = Vec::with_capacity(self.batteries.len());

        for &digit in &self.batteries {
            while !stack.is_empty() && (to_remove > 0) && (*stack.last().unwrap() < digit) {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(digit);
        }

        // Keep only the first `num_batteries` digits to get exact length
        let chosen = &stack[..num_batteries];

        // Turn the digits into a base‑10 number
        let combined = chosen.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        Ok(combined)
    }
}

fn get_banks<const N: usize>(input: &str) -> Result<Vec<Bank<N>>, String> {
    // Since the input will be the same every time...
    const NUM_INPUT_LINES: usize = 200;

    let mut banks = Vec::<Bank<N>>::with_capacity(NUM_INPUT_LINES);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let bank = Bank::from_string(line).map_err(|e| {
            format!("Received an error while parsing the line below: {e}:\n  {line}")
        })?;
        banks.push(bank);
    }

    banks.shrink_to_fit();

    Ok(banks)
}

fn part_1(input: &str) -> Result<u64, String> {
    let banks = get_banks::<100>(input)?;
    let mut acc = 0u64;
    for bank in &banks {
        acc += bank.joltage(2)?;
    }
    Ok(acc)
}

fn part_2(input: &str) -> Result<u64, String> {
    let banks = get_banks::<100>(input)?;
    let mut acc = 0u64;
    for bank in &banks {
        acc += bank.joltage(12)?;
    }
    Ok(acc)
}

pub fn p3_1() {
    match part_1(super::input_to_string(3, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 03 Part 1: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 03 Part 1: ERROR: {}", e);
        }
    }
}

pub fn p3_2() {
    match part_2(super::input_to_string(3, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 03 Part 2: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 03 Part 2: ERROR: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn parse_banks() {
        let banks = get_banks::<15>(EXAMPLE).unwrap();
        assert_eq!(banks.len(), 4);
        assert_eq!(
            banks[0].batteries,
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            banks[1].batteries,
            [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
        );
        assert_eq!(
            banks[2].batteries,
            [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        );
        assert_eq!(
            banks[3].batteries,
            [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
    }

    #[test]
    fn part_1() {
        let banks = get_banks::<15>(EXAMPLE).unwrap();
        assert_eq!(banks.len(), 4);
        assert_eq!(banks[0].joltage(2), Ok(98));
        assert_eq!(banks[1].joltage(2), Ok(89));
        assert_eq!(banks[2].joltage(2), Ok(78));
        assert_eq!(banks[3].joltage(2), Ok(92));
    }

    #[test]
    fn part_2() {
        let banks = get_banks::<15>(EXAMPLE).unwrap();
        assert_eq!(banks.len(), 4);
        assert_eq!(banks[0].joltage(12), Ok(987654321111));
        assert_eq!(banks[1].joltage(12), Ok(811111111119));
        assert_eq!(banks[2].joltage(12), Ok(434234234278));
        assert_eq!(banks[3].joltage(12), Ok(888911112111));
    }
}
