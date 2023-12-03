#[macro_use]
extern crate lazy_static;

mod part1;
use std::str::FromStr;

use advent::read_input_for_day;
use part1::{AdventError, CalibrationDocument};

fn main() -> Result<(), AdventError> {
  let input = read_input_for_day(1).unwrap();
  let doc = CalibrationDocument::from_str(&input)?;

  println!("[day-1][part-1] The sum of all lines is: {}", doc.sum());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

  #[test]
  fn day_one_part_one() {
    let doc = CalibrationDocument::from_str(INPUT).unwrap();
    assert_eq!(doc.sum(), 142)
  }
}
