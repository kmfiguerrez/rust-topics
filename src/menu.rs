use std::io::{self, Write};
use thiserror::Error;
use std::process::Command;


pub fn user_input() -> Result<u8, UserInputError> {
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
          return Err(UserInputError::Quit);
      }
      // println!("You entered: {}", input);
      // println!("{n} bytes read");
    }
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      return Err(UserInputError::Io(err));
    }
  }

  // Parse user input into integers.
  let input: u8 = match input.trim().parse() {
    Ok(num) => num,
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      // println!("Program terminated! You did not enter an integer!");
      return Err(UserInputError::Parse(err));
    }
  };
  Ok(input)
}


// This code uses the thiserror library.
#[derive(Error, Debug)]
pub enum UserInputError {
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