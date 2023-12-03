use advent::parse_from_captures_or;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum AdventError {
  InvalidRegex,
  NoFirstDigit,
}

pub struct CalibrationDocument {
  pub readings: Vec<Reading>,
}

impl FromStr for CalibrationDocument {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let readings = s
      .lines()
      .map(|l: &str| l.parse().unwrap())
      .collect::<Vec<Reading>>();

    return Ok(CalibrationDocument { readings });
  }
}

impl CalibrationDocument {
  pub fn sum(&self) -> i32 {
    self.readings.iter().fold(0, |acc, next| acc + next.value)
  }
}

pub struct Reading {
  pub value: i32,
}

impl FromStr for Reading {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref FIRST_RE: Regex = Regex::new(r"^[a-z]*(\d)").unwrap();
      static ref LAST_RE: Regex = Regex::new(r"(\d)[a-z]*$").unwrap();
    }

    let first_captures = FIRST_RE.captures(s).ok_or(AdventError::InvalidRegex)?;
    let last_captures = LAST_RE.captures(s).ok_or(AdventError::InvalidRegex)?;

    let first_digit =
      parse_from_captures_or::<i32, AdventError>(&first_captures, 1, AdventError::NoFirstDigit)?;
    let last_digit =
      parse_from_captures_or::<i32, AdventError>(&last_captures, 1, AdventError::NoFirstDigit)?;

    let value = format!("{}{}", first_digit, last_digit).parse().unwrap();
    return Ok(Reading { value });
  }
}
