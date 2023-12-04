#[macro_use]
extern crate lazy_static;

mod models;

use std::str::FromStr;

use advent::read_input_for_day;
use models::{CardPile, Day4Error};

fn main() -> Result<(), Day4Error> {
  println!("day-4");
  let input = read_input_for_day(4).unwrap();
  let pile = CardPile::from_str(&input)?;

  println!("[day-4][part-1] The total score is {}", pile.score());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const P1_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

  #[test]
  fn day_four_part_one() {
    let pile = CardPile::from_str(P1_INPUT).unwrap();
    assert_eq!(pile.score(), 13)
  }
}
