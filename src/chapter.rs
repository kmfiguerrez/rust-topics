use crate::menu;

pub mod content;

pub struct Chapter<'a> {
  title: &'a str,
  chapter: &'a str,
  sections: Vec<Section<'a>>
}

impl<'a> Chapter<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn get_chapter(&self) -> &'a str {
    self.chapter
  }  

  pub fn display_sections(&self) {
    let mut i: u8 = 1;
    for section in &self.sections {
      println!("{}. {}", i, section.get_title());
      i+= 1;
    }
  }

  pub fn get_section(&self, selected_section: u8) -> Option<&Section<'a>> {
    if selected_section == 0 {
      None
    } else {
      self.sections.get((selected_section - 1) as usize)
    }
  }

  // Associate functions.
  pub fn new(title: &'a str, chapter: &'a str, sections: Vec<Section<'a>>) -> Self {
    Self {
      title,
      chapter,
      sections
    }
  }
}

pub struct Section<'a> {
  title: &'a str,
  section: &'a str,
  content: fn(title: &str, section: &str)
}

impl<'a> Section<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn get_section(&self) -> &'a str {
    self.section
  }  

  pub fn display_content(&self) {
    (self.content)(self.get_title(), self.get_section());
  }

  // Associate Functions.
  pub fn new(title: &'a str, section: &'a str, content: fn(&str, &str)) -> Self {
    Self {
      title,
      section,
      content
    }
  }
}


pub struct SubHeader<'a> {
  title: &'a str,
  content: fn()
}

impl<'a> SubHeader<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_content(&self) {
    (self.content)();
  }

  // Associate Functions.
  pub fn new(title: &'a str, content: fn()) -> Self {
    Self {
      title,
      content
    }
  }

  /// Prompt for subheaders.
  /// 
  /// This associate function list the available subheaders for a user to select
  /// and display the content of the selected subheader.
  /// 
  /// # Arguments
  /// 
  /// * `subheaders` - An Array of SubHeader to display.
  /// * `section_title` - The title of the selected section.
  pub fn prompt_subheader(subheaders: &[SubHeader], section_title: &str, section: &str) {
    loop {
      menu::section_title(section_title, section);
      
      let mut i:u8 = 1;
      for subheader in subheaders {
        println!("{}. {}",i, subheader.get_title());
        i+= 1;
      }

      println!();

      'prompting_header_loop: loop {
        let selected_number = menu::post_menu_prompt();
        let selected_number: u8 = match selected_number {
          Ok(menu::PostMenuPromptAction::ListPreviousMenu) => {
            menu::clear_screen();
            return;
          }
          Ok(menu::PostMenuPromptAction::Quit) => std::process::exit(0),
          Ok(menu::PostMenuPromptAction::Integer(int_input)) => {
            if int_input == 0 {continue;}
            if int_input as usize > subheaders.len() {
              continue;
            }
            int_input
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
        };

        // Display selected header content.
        for (index, value) in subheaders.iter().enumerate() {
          if (selected_number as usize) - 1 == index {
            menu::clear_screen();
            value.display_content();
            break;
          }
        }

        loop {
          match menu::post_header_prompt() {
            Ok(menu::PostHeaderPromptAction::ListPreviousMenu) => {
              menu::clear_screen();
              break 'prompting_header_loop;
            }
            Ok(menu::PostHeaderPromptAction::Quit) => std::process::exit(0),
            Err(menu::PostHeaderPromptError::InvalidOption(_)) => continue,
            Err(menu::PostHeaderPromptError::Io(err)) => {
              eprintln!("I/O error: {err}");
              std::process::exit(1);
            }
          }
        }
      };
    }
  }

}