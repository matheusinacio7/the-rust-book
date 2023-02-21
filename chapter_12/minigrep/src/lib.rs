use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = fs::read_to_string(config.file_path)?;

  for (i, line) in file_contents.lines().enumerate() {
      println!("Line {}: {}", i, line);
  }

  Ok(())
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
