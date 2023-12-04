use advent::parse_from_captures_or;
use regex::{Captures, Regex};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Day4Error {
  InvalidRegex,
}

pub struct CardPile {
  cards: Vec<ScratchCard>,
}

impl CardPile {
  pub fn score(&self) -> i32 {
    self.cards.iter().fold(0, |acc, card| acc + card.score())
  }
}

impl FromStr for CardPile {
  type Err = Day4Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cards = s
      .lines()
      .map(|c| c.parse().unwrap())
      .collect::<Vec<ScratchCard>>();

    Ok(CardPile { cards })
  }
}

pub struct ScratchCard {
  winning_numbers: Vec<i32>,
  your_numbers: Vec<i32>,
}

impl ScratchCard {
  pub fn score(&self) -> i32 {
    self.your_numbers.iter().fold(0, |acc, num| {
      match (acc, self.winning_numbers.contains(num)) {
        (0, true) => 1,
        (_, true) => acc * 2,
        _ => acc,
      }
    })
  }
}

fn captures_to_i32_vec(captures: &Captures, index: usize) -> Result<Vec<i32>, Day4Error> {
  let vec = parse_from_captures_or::<String, Day4Error>(captures, index, Day4Error::InvalidRegex)?
    .split(' ')
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  Ok(vec)
}

impl FromStr for ScratchCard {
  type Err = Day4Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex =
        Regex::new(r"^Card\s+\d+:\s+(\d+(?:\s+\d+)*)\s+\|\s+(\d+(?:\s+\d+)*)").unwrap();
    }

    let captures = RE.captures(s).ok_or(Day4Error::InvalidRegex)?;
    let winning_numbers = captures_to_i32_vec(&captures, 1)?;
    let your_numbers = captures_to_i32_vec(&captures, 2)?;

    Ok(ScratchCard {
      winning_numbers,
      your_numbers,
    })
  }
}
