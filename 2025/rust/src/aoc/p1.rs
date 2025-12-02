const STARTING_POSITION: u32 = 50;
const NUM_POSITIONS: u32 = 100;

#[derive(Debug, Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct DialTurn {
    direction: TurnDirection,
    amount: u32,
}

impl DialTurn {
    fn from_instruction(instruction: &str) -> DialTurn {
        let lazy_error = || format!("Invalid instruction: {instruction}");

        let mut chars = instruction.chars();

        // Left or right? Read the first character
        let direction_char = chars.next().expect(lazy_error().as_str());
        let direction = match direction_char {
            'L' => TurnDirection::Left,
            'R' => TurnDirection::Right,
            _ => panic!("{}", lazy_error().as_str()),
        };

        // The rest of the instruction should just be a number
        let amount: u32 = chars
            .collect::<String>()
            .parse()
            .expect(lazy_error().as_str());

        DialTurn { direction, amount }
    }
}

fn turn_dial(dial: u32, password: &mut u32, turn: &DialTurn, count_passing: bool) -> u32 {
    // Wrapped arithmetic - each case expanded for hooks
    match turn.direction {
        TurnDirection::Left => {
            if turn.amount == dial {
                *password += 1;
                0
            } else if turn.amount > dial {
                if count_passing && (dial != 0) {
                    *password += 1;
                }
                NUM_POSITIONS - (turn.amount - dial)
            } else {
                dial - turn.amount
            }
        }
        TurnDirection::Right => {
            let headroom = NUM_POSITIONS - dial;
            if turn.amount == headroom {
                *password += 1;
                0
            } else if turn.amount > headroom {
                if count_passing && (dial != 0) {
                    *password += 1;
                }
                turn.amount - headroom
            } else {
                dial + turn.amount
            }
        }
    }
}

/// Read turns from the given input file
fn get_turns() -> Vec<DialTurn> {
    let input = super::input_to_string(1, 1);

    input
        .lines()
        .map(|line| DialTurn::from_instruction(line))
        .collect()
}

/// Solve the puzzle with `turns` parsed from text input. `count_passing` controls whether part 2 is
/// being solved or not.
fn solve(turns: &Vec<DialTurn>, count_passing: bool) -> u32 {
    let mut dial = STARTING_POSITION;
    let mut password: u32 = 0;

    for turn in turns {
        if count_passing {
            password += turn.amount / NUM_POSITIONS;
        }

        let truncated_turn = DialTurn {
            direction: turn.direction,
            amount: turn.amount % NUM_POSITIONS,
        };

        dial = turn_dial(dial, &mut password, &truncated_turn, count_passing)
    }
    password
}

/// Problem 1 Part 1
pub fn p1_1() {
    println!("Problem 01 Part 01: {}", solve(&get_turns(), false));
}

/// Problem 1 Part 2
pub fn p1_2() {
    println!("Problem 01 Part 02: {}", solve(&get_turns(), true));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example given on https://adventofcode.com/2025/day/1
    const EXAMPLE: [&str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn parse_instructions() {
        let one_digit = DialTurn::from_instruction("L5");
        assert!(matches!(one_digit.direction, TurnDirection::Left));
        assert_eq!(one_digit.amount, 5);

        let two_digits = DialTurn::from_instruction("R42");
        assert!(matches!(two_digits.direction, TurnDirection::Right));
        assert_eq!(two_digits.amount, 42);

        let three_digits = DialTurn::from_instruction("L123");
        assert!(matches!(three_digits.direction, TurnDirection::Left));
        assert_eq!(three_digits.amount, 123);

        let four_digits = DialTurn::from_instruction("R9999");
        assert!(matches!(four_digits.direction, TurnDirection::Right));
        assert_eq!(four_digits.amount, 9999);
    }

    #[test]
    fn example() {
        let turns: Vec<DialTurn> = EXAMPLE
            .iter()
            .map(|instruction| DialTurn::from_instruction(instruction))
            .collect();
        assert_eq!(solve(&turns, false), 3);
        assert_eq!(solve(&turns, true), 6);
    }

    #[test]
    fn large_spins() {
        let turns = vec![
            DialTurn::from_instruction("L25"),  // 25
            DialTurn::from_instruction("R75"),  // 0
            DialTurn::from_instruction("L525"), // 75 (pass 5 times)
            DialTurn::from_instruction("R750"), // 25 (pass 8 times)
            DialTurn::from_instruction("L15"),  // 40
            DialTurn::from_instruction("R999"), // 39 (pass 10 times)
        ];
        assert_eq!(solve(&turns, false), 1);
        assert_eq!(solve(&turns, true), 24);
    }
}
