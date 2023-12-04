use advent::parse_from_captures_or;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Part2Error {
  InvalidRegex,
  NoFirstDigit,
}

pub struct EnhancedCalibrationDocument {
  pub readings: Vec<EnhancedReading>,
}

impl FromStr for EnhancedCalibrationDocument {
  type Err = Part2Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let readings = s
      .lines()
      .map(|l: &str| l.parse().unwrap())
      .collect::<Vec<EnhancedReading>>();

    return Ok(EnhancedCalibrationDocument { readings });
  }
}

impl EnhancedCalibrationDocument {
  pub fn sum(&self) -> i32 {
    self.readings.iter().fold(0, |acc, next| acc + next.value)
  }
}

pub struct EnhancedReading {
  pub value: i32,
}

impl FromStr for EnhancedReading {
  type Err = Part2Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref FIRST_RE: Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero|\d)(?:[a-z]|\d)*$")
          .unwrap();
      static ref LAST_RE: Regex =
        Regex::new(r"^(?:[a-z]|\d)*(one|two|three|four|five|six|seven|eight|nine|zero|\d)")
          .unwrap();
    }

    let first_captures = FIRST_RE.captures(s).ok_or(Part2Error::InvalidRegex)?;
    let last_captures = LAST_RE.captures(s).ok_or(Part2Error::InvalidRegex)?;

    let first_digit =
      parse_from_captures_or::<String, Part2Error>(&first_captures, 1, Part2Error::NoFirstDigit)?;
    let last_digit =
      parse_from_captures_or::<String, Part2Error>(&last_captures, 1, Part2Error::NoFirstDigit)?;

    let value = format!(
      "{}{}",
      digit_for_match(&first_digit),
      digit_for_match(&last_digit)
    )
    .parse()
    .unwrap();
    return Ok(EnhancedReading { value });
  }
}

fn digit_for_match(str: &str) -> i32 {
  match str {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    "zero" => 0,
    digit => digit.parse().unwrap(),
  }
}
