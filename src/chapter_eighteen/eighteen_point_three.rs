use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Encoding States and Behavior as Types", esabat_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Implementing an Object-Oriented Design Pattern");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#implementing-an-object-oriented-design-pattern".bright_cyan(),
  )
}

// Header: Encoding States and Behavior as Types. Abbreviated as esabat.
fn esabat_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Encoding States and Behavior as Types");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types".bright_cyan(),
  )
}














