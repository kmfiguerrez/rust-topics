// use owo_colors::OwoColorize;

use std::io::{self, Write};

fn main() {
// user_input();
rust_topics::chapter_four::four_point_one::display_contents();

}

fn user_input() {
  loop {
    print!("Enter input (q to quit): ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(0) => {
        // EOF (Ctrl+D/Ctrl+Z) — exit
        println!("Read 0 bytes");
        break;
      }
      Ok(n) => {
        let input = input.trim();
        if input == "q" {
            println!("Quitting.");
            break;
        }
        println!("You entered: {}", input);
        println!("{n} bytes read");
      }
      Err(err) => {
        eprintln!("Error reading input: {}", err);
        break;
      }
    }
  }
}

