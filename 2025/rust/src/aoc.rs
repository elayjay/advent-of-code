use std::fs::read_to_string;
use std::path::Path;

pub mod p1;
pub use p1::*;

pub mod p2;
pub use p2::*;

pub mod p3;
pub use p3::*;

pub mod p4;
pub use p4::*;

pub mod p5;
pub use p5::*;

pub mod p6;
pub use p6::*;

fn input_to_string(problem: i8, part: i8) -> String {
    let instructions = "\
        This project is intended to be built and run with Cargo, as it depends on the \
        `CARGO_MANIFEST_DIR` environment variable for getting the path for puzzle input. See the \
        README in each language directory for more information.";

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(instructions)
        .join("input")
        .join(format!("{}.{}.txt", problem, part));

    let path_str = path
        .to_str()
        .expect("Environment variable CARGO_MANIFEST_DIR points to an invalid path.");

    read_to_string(&path).expect(
        format!("Puzzle input was not found at {path_str}. Download your puzzle input to the specified file.").as_str(),
    )
}
