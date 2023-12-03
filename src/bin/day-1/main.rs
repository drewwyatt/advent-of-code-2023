fn main() -> std::io::Result<()> {
  println!("Hello, world!");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_one_part_one() {
    assert_eq!(true, true)
  }
}
