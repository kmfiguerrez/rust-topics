use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Writing a Failing Test for Case-Insensitive Search", waftfcis_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  menu::subheader_title("Section Introduction");

  println!(
  "We'll improve the minigrep binary by adding an extra feature: an option for case-insensitive searching that the user \
  can turn on via an environment variable.\n\
  We could make this feature a command line option and require that users enter it each time they want it to apply, \
  but by instead making it an environment variable, we allow our users to set the environment variable once and have \
  all their searches be case insensitive in that terminal session.
  ")
}

// Header: Writing a Failing Test for Case-Insensitive Search. Abbreviated as waftfcis.
fn waftfcis_content() {
  menu::subheader_title("Writing a Failing Test for Case-Insensitive Search");

  println!(
  "We first add a new {0} function to the {1} library that will be called when the environment variable \
  has a value.\n\
  We'll continue to follow the TDD process, so the first step is again to write a failing test.\n\
  We'll add a new test for the new {0} function and rename our old test from {2} to {3} to \
  clarify the differences between the two tests.\n\n\
  See Listing 12-20: {4}, for code sample and explanation.\n\n\
  ",
  "search_case_insensitive".bright_yellow().bold(),
  "minigrep".bright_cyan().bold(),
  "one_result".bright_blue().bold(),
  "case_sensitive".bright_green().bold(),
  "https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html#listing-12-20".cyan()
  )
}







