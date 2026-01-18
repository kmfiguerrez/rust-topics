// use owo_colors::OwoColorize;

use std::io::{self, Write};
use rust_topics::{chapter, chapter_four, chapter_six, menu};

fn main() {
  // rust_topics::chapter_four::four_point_one::display_contents();
  let chapters: [rust_topics::chapter::Chapter; 2];
  chapters = [
    chapter::Chapter::new(
      "Understanding Ownership",
      "Chapter 4",
      chapter_four::section::generate_sections()
    ),
    chapter::Chapter::new(
      "Enums and Pattern Matching",
      "Chapter 6",
      chapter_six::section::generate_sections()
    )
  ];
  
  loop {
    println!("Rust topics\n");

    let mut line_number: u8 = 1;
    for chapter in &chapters {
      println!("{}. {}: {}", line_number, chapter.get_chapter(), chapter.get_title());
      line_number += 1;
    }

    println!();

    'prompting_chapter_loop: loop {
      let selected_number = menu::integer_prompt();
      let selected_number = match selected_number {
        Ok(num) => {
          // println!("You selected {num}");
          if num as usize > chapters.len() {
            continue;
          }
          num
        }
        Err(menu::IntegerPromptError::Quit) => {
          println!("Exiting program safely...");
          std::process::exit(0);   
        }
        Err(menu::IntegerPromptError::Io(err)) => {
          eprintln!("I/O error: {err}");
          return;
        }
        Err(menu::IntegerPromptError::Parse(err)) => {
          eprintln!("Parse error: {err}");
          // println!("Select an integer!");
          continue;
        }
      };

      // Assigned temporary values to both selected_chapter and selected_section.
      let mut selected_chapter = &chapters[0];
      let mut selected_section: u8 = 0;
      if selected_number == 1 {
        menu::clear_screen();
        selected_chapter = &chapters[(selected_number as usize) - 1];

        // chapters[(selected_number as usize) - 1].display_sections();
      }
      else if selected_number == 2 {
        menu::clear_screen();
        // chapters[(selected_number as usize) - 1].display_sections();
        selected_chapter = &chapters[(selected_number as usize) - 1];
      };

      selected_chapter.display_sections();



      loop {
        match menu::post_section_prompt() {
          Ok(menu::PostSubMenuAction::ListPreviousMenu) => {
            menu::clear_screen();
            break 'prompting_chapter_loop;
          }
          Ok(menu::PostSubMenuAction::Quit) =>std::process::exit(0),
          Err(menu::PostSubMenuError::InvalidOption(_)) => continue,
          Err(menu::PostSubMenuError::Io(err)) => {
            eprintln!("I/O error: {err}");
            return;
          }
        }
      }    
    }
  }

  // chapters[0].display_sections();
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

