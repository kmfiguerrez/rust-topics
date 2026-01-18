// use owo_colors::OwoColorize;

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
  
  // This outer loop will display the chapters and then proceed to selecting a
  // chapter then to section down to headers.
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

      // This part is about to select a chapter.
      // Initialized temporary values to both selected_chapter and selected_section.
      let mut selected_chapter = &chapters[0];
      let mut selected_section: &chapter::Section;
      if selected_number == 1 {
        menu::clear_screen();
        selected_chapter = &chapters[(selected_number as usize) - 1];
      }
      else if selected_number == 2 {
        menu::clear_screen();
        selected_chapter = &chapters[(selected_number as usize) - 1];
      };
      
      // This part is about to select a section under a selected chapter.
      loop {
        selected_chapter.display_sections();
        loop {
          match menu::post_menu_prompt() {
            Ok(menu::PostMenuPromptAction::ListPreviousMenu) => {
              menu::clear_screen();
              break 'prompting_chapter_loop;
            }
            Ok(menu::PostMenuPromptAction::Quit) => std::process::exit(0),
            Ok(menu::PostMenuPromptAction::Integer(int_input)) => {
              // the int_input inside the Integer variant means the selected_section.
              match selected_chapter.get_section(int_input) {
                Some(section) => {
                  selected_section = section;
                  break;
                }
                None => continue
              }
            }
            Err(menu::PostMenuPromptError::InvalidOption(_)) => continue,
            // Err(menu::PostMenuPromptError::Parse(err)) => {
            //   eprintln!("Parse error: {err}");
            //   std::process::exit(1);
            // }
            Err(menu::PostMenuPromptError::Io(err)) => {
              eprintln!("I/O error: {err}");
              std::process::exit(1);
            }
          }
        }
        // This part is about to select a header under a selected section.
        selected_section.display_content();
      }
    }
  }
}



