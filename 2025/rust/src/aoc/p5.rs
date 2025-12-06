//! Problem 5: https://adventofcode.com/2025/day/5

use std::ops::RangeInclusive;

fn range_from_str(s: &str) -> Result<RangeInclusive<u64>, String> {
    let line = s.trim();

    let mut bounds = line.split('-');
    let (Some(lower), Some(upper)) = (bounds.next(), bounds.next()) else {
        return Err(format!("Expected two numbers separated by a dash: {line}"));
    };

    let Ok(lower) = lower.parse::<u64>() else {
        return Err(format!("Lower bound is not an integer: {line}"));
    };
    let Ok(upper) = upper.parse::<u64>() else {
        return Err(format!("Upper bound is not an integer: {line}"));
    };

    Ok(RangeInclusive::new(lower, upper))
}

fn parse_ranges(s: &str) -> Result<Vec<RangeInclusive<u64>>, String> {
    let mut ranges = Vec::<RangeInclusive<u64>>::new();
    for line in s.lines() {
        let line = line.trim();
        let range = range_from_str(line)
            .map_err(|e| format!("Error while parsing range: {e}: \"{line}\""))?;
        ranges.push(range);
    }
    Ok(ranges)
}

fn parse_list(s: &str) -> Result<Vec<u64>, String> {
    let mut ranges = Vec::<u64>::new();
    for line in s.lines() {
        let line = line.trim();
        ranges.push(
            line.trim()
                .parse::<u64>()
                .map_err(|e| format!("Error while parsing number: {e}: \"{line}\""))?,
        );
    }
    Ok(ranges)
}

fn part_1(input: &str) -> Result<u64, String> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() != 2 {
        return Err(format!("Expected 2 parts but got {}", parts.len()));
    }

    let ranges = parse_ranges(parts[0])?;
    let product_list = parse_list(parts[1])?;

    let mut count = 0u64;
    for product in &product_list {
        for range in &ranges {
            if range.contains(product) {
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

fn part_2(input: &str) -> Result<u64, String> {
    let ranges_text = input
        .split("\n\n")
        .next()
        .ok_or_else(|| "Expected two parts separated by empty line".to_string())?;
    let mut ranges = parse_ranges(ranges_text)?;
    ranges.retain(|e| !e.is_empty());
    ranges.sort_by_key(|e| *e.start());

    let mut merged = Vec::<RangeInclusive<u64>>::new();
    for r in ranges {
        match merged.last_mut() {
            Some(last) => {
                if *r.start() <= last.end() + 1 {
                    // Overlapping or adjacent - combine into one range
                    let new_end = (*last.end()).max(*r.end());
                    let start = *last.start();
                    *last = start..=new_end;
                } else {
                    merged.push(r);
                }
            }
            None => {
                merged.push(r);
            }
        }
    }

    let total_coverage = merged.iter().fold(0, |acc, x| acc + (x.end() - x.start() + 1));
    Ok(total_coverage)
}

pub fn p5_1() {
    match part_1(super::input_to_string(5, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 05 Part 1: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 05 Part 1: ERROR: {}", e);
        }
    }
}

pub fn p5_2() {
    match part_2(super::input_to_string(5, 1).as_str()) {
        Ok(answer) => {
            println!("Problem 05 Part 2: {}", answer);
        }
        Err(e) => {
            eprintln!("Problem 05 Part 2: ERROR: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::{formatdoc, indoc};

    const EXAMPLE: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn parse_ranges() {
        let input = formatdoc! {"
            0-0
            1-3
            47389-473214324
            {}-{}
            ",
            u64::MIN, u64::MAX,
        };
        let expected = vec![0u64..=0, 1u64..=3, 47389..=473214324, u64::MIN..=u64::MAX];
        assert_eq!(super::parse_ranges(input.as_str()), Ok(expected));
    }

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(EXAMPLE), Ok(3));
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(EXAMPLE), Ok(14));
    }
}
