use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Writing a Failing Test for Case-Insensitive Search", waftfcis_content),
    chapter::SubHeader::new("Implementing the search_case_insensitive Function", itsf_content),
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

// Header: Implementing the search_case_insensitive Function. Abbreviated as itsf.
fn itsf_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing the search_case_insensitive Function");

  println!(
  "The {0} function, shown in Listing 12-21, will be almost the same as the {1} function.\n\
  The only difference is that we'll lowercase the {2} and each {3} so that whatever the case of the input arguments, \
  they'll be the same case when we check whether the line contains the query.\n\n\
  See Listing 12-21:{4}, for code sample, explanation and code outputs and complete reading.\n\n\
  You can set the environment variable in your terminal at the same time as you run the program.\n\n\
  {5}\n\n\
  In powershell:\n\
  {6}\n\n\
  This will make {7} persist for the remainder of your shell session.\n\
  It can be unset with the {8} cmdlet: {9}
  ",
  "search_case_insensitive".bright_yellow().bold(),
  "search".bright_yellow().bold(),
  "query".bright_yellow().bold(),
  "line".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html#listing-12-21".cyan(),
  "IGNORE_CASE=1 cargo run -- to poem.txt".bright_yellow().bold(),
  "$Env:IGNORE_CASE=1; cargo run -- to poem.txt".bright_yellow().bold(),
  "IGNORE_CASE".bright_yellow().bold(),
  "Remove-Item".bright_yellow().bold(),
  "Remove-Item Env:IGNORE_CASE".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The functions for working with environment variables are in the {1} module in the standard library.\n\
  {solid_disc} We'll use the {2} function from {1} module to check to see if any value has been set for an environment variable.\n\
  {solid_disc} You can call the {3} on the {4} to map the success case to {5} and error case to {6}. It is usually used if you \
  don't care about the values they hold. For example, to just check whether the environment variable is set.
  ",
  "REMEMBER".bright_white().bold(),
  "std::env".bright_yellow().bold(),
  "var".bright_yellow().bold(),
  "is_ok".bright_yellow().bold(),
  "Result".bright_yellow().bold(),
  "true".bright_yellow().bold(),
  "false".bright_yellow().bold()
)

}






