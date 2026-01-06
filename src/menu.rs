use std::{io::{self, Write}, num::ParseIntError};

pub fn user_input() -> Result<u8, ParseIntError> {
  print!("Select a number (q to quit): ");
  io::stdout().flush().expect("failed to flush stdout");

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(0) => {
      // EOF (Ctrl+D/Ctrl+Z) — exit
      println!("Read 0 bytes");
    }
    Ok(n) => {
      let input = input.trim();
      if input == "q" {
          println!("Quitting.");
      }
      println!("You entered: {}", input);
      println!("{n} bytes read");
    }
    Err(err) => {
      eprintln!("Error reading input: {}", err);
    }
  }

  // Parse user input into integers.
  let input: u8 = match input.trim().parse() {
    Ok(num) => num,
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      println!("Program terminated! You did not enter an integer!");
      return Err(err)
    }
  };
  Ok(input)
}