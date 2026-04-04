use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction: Performance in Loops vs. Iterators");

  println!(
  "See: {}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch13-04-performance.html#performance-in-loops-vs-iterators".cyan()

  )
}