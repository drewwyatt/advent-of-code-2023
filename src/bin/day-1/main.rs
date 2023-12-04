#[macro_use]
extern crate lazy_static;

mod part1;
mod part2;
use std::str::FromStr;

use advent::read_input_for_day;
use part1::{CalibrationDocument, Part1Error};
use part2::{EnhancedCalibrationDocument, Part2Error};

#[derive(Debug)]
enum Day1Error {
  Part1(Part1Error),
  Part2(Part2Error),
}

impl From<Part1Error> for Day1Error {
  fn from(e: Part1Error) -> Self {
    Day1Error::Part1(e)
  }
}

impl From<Part2Error> for Day1Error {
  fn from(e: Part2Error) -> Self {
    Day1Error::Part2(e)
  }
}

fn main() -> Result<(), Day1Error> {
  let input = read_input_for_day(1).unwrap();
  let doc1 = CalibrationDocument::from_str(&input)?;
  let doc2 = EnhancedCalibrationDocument::from_str(&input)?;

  println!("[day-1][part-1] The sum of all lines is: {}", doc1.sum());
  println!("[day-1][part-2] The sum of all lines is: {}", doc2.sum());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  const P1_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

  const P2_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

  #[test]
  fn day_one_part_one() {
    let doc = CalibrationDocument::from_str(P1_INPUT).unwrap();
    assert_eq!(doc.sum(), 142)
  }

  #[test]
  fn day_one_part_two() {
    let doc = EnhancedCalibrationDocument::from_str(P2_INPUT).unwrap();
    assert_eq!(doc.sum(), 281)
  }
}
