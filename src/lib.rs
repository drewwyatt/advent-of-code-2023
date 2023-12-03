use regex::Captures;
use std::fs;
use std::str::FromStr;

pub fn read_input_for_day(day: usize) -> std::io::Result<String> {
  let path = format!("./src/bin/day-{}/INPUT.txt", day);
  fs::read_to_string(path)
}

pub fn read_input_for_day_as<T>(day: usize) -> std::io::Result<Vec<T>>
where
  T: FromStr,
  T::Err: std::fmt::Debug,
{
  let contents = read_input_for_day(day)?;
  let values = contents
    .lines()
    .map(|l| l.parse().unwrap())
    .collect::<Vec<T>>();

  Ok(values)
}

pub fn parse_from_captures_or<T, U>(captures: &Captures, index: usize, err: U) -> Result<T, U>
where
  T: FromStr,
  U: Copy,
{
  captures
    .get(index)
    .ok_or(err)?
    .as_str()
    .parse::<T>()
    .map_err(|_| err)
}

pub fn parse_from_named_captures_or<T, U>(captures: &Captures, name: &str, err: U) -> Result<T, U>
where
  T: FromStr,
  U: Copy,
{
  captures
    .name(name)
    .ok_or(err)?
    .as_str()
    .parse::<T>()
    .map_err(|_| err)
}
