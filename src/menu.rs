use std::io::{self, Write};
use thiserror::Error;
use std::process::Command;


pub fn headers_prompt() -> Result<u8, HeaderPromptError> {
  print!("Select a number (q to quit): ");
  io::stdout().flush().expect("failed to flush stdout");

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(0) => {
      // EOF (Ctrl+D/Ctrl+Z) — exit
      // println!("Read 0 bytes");
    }
    Ok(_) => {
      let input = input.trim();
      if input == "q" {
        // println!("Quitting.");
        return Err(HeaderPromptError::Quit);
      }
      // println!("You entered: {}", input);
      // println!("{n} bytes read");
    }
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      return Err(HeaderPromptError::Io(err));
    }
  }

  // Parse user input into integers.
  let input: u8 = match input.trim().parse() {
    Ok(num) => num,
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      // println!("Program terminated! You did not enter an integer!");
      return Err(HeaderPromptError::Parse(err));
    }
  };
  Ok(input)
}


// This code uses the thiserror library.
#[derive(Error, Debug)]
pub enum HeaderPromptError {
    #[error("User chose to quit")]
    Quit, // Our new variant

    #[error("IO error: {0}")]
    Io(#[from] io::Error), // Handles flush and read_line

    #[error("Parsing error: {0}")]
    Parse(#[from] std::num::ParseIntError), // Handles parse
}

pub fn clear_screen() {
  // Clear previous screen.
  if cfg!(target_os = "windows") {
      Command::new("cmd").args(["/c", "cls"]).status().unwrap();
  } else {
      Command::new("clear").status().unwrap();
  }       
}

// Action returned when reading the small menu input.
pub enum PostHeaderPromptAction {
  Quit,
  ListSubheaders,
}

// Error type for the small menu input reader. Uses `thiserror` for nice messages.
#[derive(Error, Debug)]
pub enum PostHeaderPromptError {
  #[error("invalid option: {0}")]
  InvalidOption(String),

  #[error("IO error: {0}")]
  Io(#[from] io::Error),
}

/// Reads a single menu command from the user and returns a `PostHeaderPromptAction`.
/// Only accepts `q` (quit) or `s` (list subheaders). Any other input is an error.
pub fn post_header_prompt() -> Result<PostHeaderPromptAction, PostHeaderPromptError> {
  print!("Enter 'q' to quit or 's' to list subheaders: ");
  io::stdout().flush()?;

  let mut input = String::new();
  let n = io::stdin().read_line(&mut input)?;
  if n == 0 {
    return Err(PostHeaderPromptError::InvalidOption("EOF".into()));
  }

  match input.trim() {
    "q" | "Q" => Ok(PostHeaderPromptAction::Quit),
    "s" | "S" => Ok(PostHeaderPromptAction::ListSubheaders),
    other => Err(PostHeaderPromptError::InvalidOption(other.to_string())),
  }
}

