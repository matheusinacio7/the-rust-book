use std::error::Error;
use std::{fs, env, io::{self, BufRead}};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file = fs::File::open(&config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.pattern, &file)?
  } else {
    search(&config.pattern, &file)?
  };

  for (i, line) in results.iter().enumerate() {
      println!("Line {}: {}", i + 1, line);
  }

  Ok(())
}

pub fn search(pattern: &str, file: &fs::File) -> Result<Vec<String>, std::io::Error> {
  let mut lines: Vec<String> = vec![];
  let buffer = io::BufReader::new(file);
  for maybeline in buffer.lines() {
    match maybeline {
      Ok(line) => {
        if line.contains(pattern) {
          lines.push(line);
        }
      },
      Err(err) => { return Err(err) },
    }
  }
  Ok(lines)
}

pub fn search_case_insensitive(pattern: &str, file: &fs::File) -> Result<Vec<String>, std::io::Error>  {
  let mut lines: Vec<String> = vec![];
  let pattern = pattern.to_lowercase();
  let buffer = io::BufReader::new(file);
  for maybeline in buffer.lines() {
    match maybeline {
      Ok(line) => {
        if line.to_lowercase().contains(&pattern) {
          lines.push(line);
        }
      },
      Err(err) => { return Err(err) },
    }
  }
  Ok(lines)
}

pub struct Config {
  pub pattern: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
      args.next();

      let pattern = args.next().ok_or("Must inform pattern")?.clone();
      let file_path = args.next().ok_or("Must inform a file path")?.clone();

      let ignore_case = dbg!(env::var("IGNORE_CASE").is_ok());

      Ok(Config { pattern, file_path, ignore_case })
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn case_sensitive() {
//     let query = "duct";
//     let o_contents = 
//     let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//     assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//   }

//   #[test]
//   fn case_insensitive() {
//     let query = "rUsT";
//     let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//     assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
//   }
// }

