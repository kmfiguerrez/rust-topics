use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction");

  println!(
    "See Listing 12-4: {0}, for code sample.\n\
    To read files, We need {1} to handle files.\n\
    In {2}, the new statement {3} takes the {4}, opens that file, and returns a value of type \
    {5} that contains the file's contents.\n\n\
    The code in Listing 12-4 has a few flaws, the {2} function has multiple responsibilities.\n\
    Generally, functions are clearer and easier to maintain if each function is responsible for only one idea.\n\
    The other problem is that we're not handling errors as well as we could.\n\
    The program is still small, so these flaws aren't a big problem, but as the program grows, \
    it will be harder to fix them cleanly. \n\
    It's a good practice to begin refactoring early on when developing a program because it's much easier \
    to refactor smaller amounts of code.
  ",
    "https://doc.rust-lang.org/book/ch12-02-reading-a-file.html#listing-12-4".cyan(),
    "std::fs".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "fs::read_to_string".bright_yellow().bold(),
    "file_path".bright_yellow().bold(),
    "std::io::Result<String>".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} Generally, functions are clearer and easier to maintain if each function is responsible for only one idea.\n\
    {solid_disc} It's a good practice to begin refactoring early on when developing a program because it's much easier to \
    refactor smaller amounts of code.
  ",
    "REMEMBER".bright_white().bold(),
  )
}