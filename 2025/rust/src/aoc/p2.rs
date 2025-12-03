//! Problem 2: https://adventofcode.com/2025/day/2

use std::ops::Range;

trait DigitCount {
    fn digit_count(&self) -> u64;
}

impl DigitCount for u64 {
    fn digit_count(&self) -> u64 {
        self.to_string().len() as u64
    }
}

fn parse_ranges(input: &str) -> Vec<Range<u64>> {
    let mut ranges = Vec::<Range<u64>>::new();

    for entry in input.split(',') {
        let lazy_error = || format!("Invalid range entry: {entry}");

        let nums = entry.trim().split('-').collect::<Vec<&str>>();

        if nums.len() != 2 {
            panic!("{}", lazy_error());
        }

        let lower: u64 = nums[0]
            .parse()
            .unwrap_or_else(|_| panic!("{}", lazy_error()));
        let upper: u64 = nums[1]
            .parse()
            .unwrap_or_else(|_| panic!("{}", lazy_error()));

        if lower > upper {
            panic!("{}", lazy_error());
        }

        if upper == u64::MAX {
            panic!("Upper bound of range must be < {}: {}", u64::MAX, entry);
        }

        ranges.push(lower..upper + 1);
    }

    ranges
}

fn is_doubled_sequence(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    s[..(len / 2)] == s[(len / 2)..len]
}

fn is_repeated_sequence(num: u64) -> bool {
    if num < 11 {
        return false;
    }

    let s = num.to_string();
    let len = s.len();
    for substr_len in 1..=(len / 2) {
        if len % substr_len != 0 {
            continue;
        }
        let candidate = &s[..substr_len];
        let mut segment_start = substr_len;
        while segment_start < len {
            if candidate != &s[segment_start..segment_start + substr_len] {
                break;
            }
            segment_start += substr_len;
        }
        if segment_start == len {
            return true;
        }
    }

    false
}

fn part_1(input: &str) -> u64 {
    let mut sum = 0u64;
    let ranges = parse_ranges(input);
    for range in ranges {
        let mut num = range.start;
        while num < range.end {
            // Numbers with an odd number of digits can't satisfy the "pair" condition - jump to
            // the next magnitude if so
            let digit_count = num.digit_count();
            if (digit_count % 2) == 1 {
                num = 10_u64.pow(digit_count as u32);
                if num >= range.end {
                    break;
                }
            }
            if is_doubled_sequence(num) {
                sum += num;
            }
            num += 1;
        }
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum = 0u64;
    let ranges = parse_ranges(input);
    for range in ranges {
        let mut num = range.start;
        while num < range.end {
            if is_repeated_sequence(num) {
                sum += num;
            }
            num += 1;
        }
    }
    sum
}

/// Problem 2 Part 1
pub fn p2_1() {
    println!(
        "Problem 02 Part 1: {}",
        part_1(super::input_to_string(2, 1).as_str())
    );
}

/// Problem 2 Part 2
pub fn p2_2() {
    println!(
        "Problem 02 Part 2: {}",
        part_2(super::input_to_string(2, 1).as_str())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example input given in https://adventofcode.com/2025/day/2
    const EXAMPLE: &str = concat!(
    "11-22,",
    "95-115,",
    "998-1012,",
    "1188511880-1188511890,",
    "222220-222224,",
    "1698522-1698528,",
    "446443-446449,",
    "38593856-38593862,",
    "565653-565659,",
    "824824821-824824827,",
    "2121212118-2121212124"
    );

    #[test]
    fn parse_ranges() {
        let ranges = super::parse_ranges(EXAMPLE);
        assert_eq!(ranges.len(), 11);
        assert_eq!(ranges[0], 11..23);
        assert_eq!(ranges[1], 95..116);
        assert_eq!(ranges[2], 998..1013);
        assert_eq!(ranges[3], 1188511880..1188511891);
        assert_eq!(ranges[4], 222220..222225);
        assert_eq!(ranges[5], 1698522..1698529);
        assert_eq!(ranges[6], 446443..446450);
        assert_eq!(ranges[7], 38593856..38593863);
        assert_eq!(ranges[8], 565653..565660);
        assert_eq!(ranges[9], 824824821..824824828);
        assert_eq!(ranges[10], 2121212118..2121212125);
    }

    #[test]
    fn digit_count() {
        assert_eq!(0_u64.digit_count(), 1);
        assert_eq!(1_u64.digit_count(), 1);
        assert_eq!(12_u64.digit_count(), 2);
        assert_eq!(123_u64.digit_count(), 3);
        assert_eq!(1234_u64.digit_count(), 4);
        assert_eq!(12345_u64.digit_count(), 5);
        assert_eq!(123456_u64.digit_count(), 6);
        assert_eq!(1234567_u64.digit_count(), 7);
        assert_eq!(12345678_u64.digit_count(), 8);
        assert_eq!(123456789_u64.digit_count(), 9);
        assert_eq!(1234567890_u64.digit_count(), 10);
        assert_eq!(u64::MAX.digit_count(), 20);
    }

    #[test]
    fn doubled_numbers() {
        assert_eq!(false, is_doubled_sequence(0));
        assert_eq!(false, is_doubled_sequence(10));
        assert_eq!(false, is_doubled_sequence(u64::MAX));

        assert_eq!(true, is_doubled_sequence(11));
        assert_eq!(true, is_doubled_sequence(2222));
        assert_eq!(true, is_doubled_sequence(9999999999));
    }

    #[test]
    fn part_1_example() {
        assert_eq!(1227775554, part_1(EXAMPLE));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(4174379265, part_2(EXAMPLE));
    }
}
