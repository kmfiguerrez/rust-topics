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
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Refutability: Whether a Pattern Might Fail to Match");

  println!(
  "Patterns come in two forms:\n\n\
  {solid_disc} refutable\n\
  {solid_disc} and irrefutable.\n\n\
  Patterns that will match for any possible value passed are irrefutable.\n\
  An example would be {0} in the statement {1} because {0} matches anything and therefore cannot fail to match.\n\
  Patterns that can fail to match for some possible value are refutable.\n\
  An example would be {2} in the expression {3} because if the value in the {4} variable is {5} rather than {6}, the {7} \
  pattern will not match.\n\n\
  Function parameters, {8} statements, and {9} loops can only accept irrefutable patterns because the program cannot do anything \
  meaningful when values don't match.\n\
  The {10} and {11} expressions and the {12} statement accept refutable and irrefutable patterns, but the compiler warns against \
  irrefutable patterns because, by definition, they're intended to handle possible failure: The functionality of a conditional is \
  in its ability to perform differently depending on success or failure.\n\n\
  In general, you shouldn't have to worry about the distinction between refutable and irrefutable patterns; however, you do need to \
  be familiar with the concept of refutability so that you can respond when you see it in an error message.\n\
  In those cases, you'll need to change either the pattern or the construct you're using the pattern with, depending on the \
  intended behavior of the code.\n\n\
  Let's look at an example of what happens when we try to use a refutable pattern where Rust requires an irrefutable pattern and \
  vice versa.\n\
  Listing 19-8 shows a {8} statement, but for the pattern, we've specified {7}, a refutable pattern.\n\
  As you might expect, this code will not compile.\n\n\
  See Listing 19-8:{13}, for code sample and complete reading.
  ",
  "x".bright_yellow().bold(),
  "let x = 5;".bright_yellow().bold(),
  "Some(x)".bright_yellow().bold(),
  "if let Some(x) = a_value".bright_yellow().bold(),
  "a_value".bright_yellow().bold(),
  "None".bright_yellow().bold(),
  "Some".bright_yellow().bold(),
  "Some(x)".bright_yellow().bold(),
  "let".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  "if let".bright_yellow().bold(),
  "while let".bright_yellow().bold(),
  "let...else".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch19-02-refutability.html#listing-19-8".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Patterns that will match for any possible value passed are {1}.\n\
  {solid_disc} Patterns that can fail to match for some possible value are {2}.\n\
  {solid_disc} Function parameters, {3} statements, and {4} loops can only accept {1} patterns because the program cannot \
  do anything meaningful when values don't match. \n\
  {solid_disc} match arms must use {2} patterns, except for the last arm, which should match any remaining values with an \
  irrefutable pattern.\n\
  {solid_disc} In general, you shouldn't have to worry about the distinction between {2} and {1} patterns; however, \
  you do need to be familiar with the concept of refutability so that you can respond when you see it in an error message.
  ",
  "REMEMBER".bright_yellow().bold(),
  "irrefutable".italic().bold(),
  "refutable".italic().bold(),
  "let".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  )
}