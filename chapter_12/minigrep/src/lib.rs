use std::error::Error;
use std::{fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.pattern, &file_contents)
  } else {
    search(&config.pattern, &file_contents)
  };

  for (i, line) in results.iter().enumerate() {
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

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut lines = vec![];
  let pattern = pattern.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&pattern) {
      lines.push(line);
    }
  }
  lines
}

pub struct Config {
  pub pattern: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
      let pattern = args.get(1).ok_or("Must inform pattern")?.clone();
      let file_path = args.get(2).ok_or("Must inform a file path")?.clone();

      let ignore_case = dbg!(env::var("IGNORE_CASE").is_ok());

      Ok(Config { pattern, file_path, ignore_case })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}

