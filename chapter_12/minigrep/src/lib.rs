use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = fs::read_to_string(config.file_path)?;

  for (i, line) in search(&config.pattern, &file_contents).iter().enumerate() {
      println!("Line {}: {}", i + 1, line);
  }

  Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut lines = vec![];
  for line in contents.lines() {
    if line.contains(pattern) {
      lines.push(line);
    }
  }
  lines
}

pub struct Config {
  pub pattern: String,
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
      let pattern = args.get(1).ok_or("Must inform pattern")?.clone();
      let file_path = args.get(2).ok_or("Must inform a file path")?.clone();

      Ok(Config { pattern, file_path })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}

