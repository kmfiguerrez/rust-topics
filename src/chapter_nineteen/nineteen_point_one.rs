use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Patterns and Matching");

  println!(
  "Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple.\n\
  Using patterns in conjunction with match expressions and other constructs gives you more control over a program's control flow.\n\
  A pattern consists of some combination of the following:\n\n\
  {solid_disc} Literals\n\
  {solid_disc} Destructured arrays, enums, structs, or tuples\n\
  {solid_disc} Variables\n\
  {solid_disc} Wildcards\n\
  {solid_disc} Placeholders\n\n\
  Some example patterns include {0}, {1}, and {2}.\n\
  In the contexts in which patterns are valid, these components describe the shape of data.\n\
  Our program then matches values against the patterns to determine whether it has the correct shape of data to continue \
  running a particular piece of code.\n\n\
  To use a pattern, we compare it to some value.\n\
  If the pattern matches the value, we use the value parts in our code.\n\
  Recall the {3} expressions in Chapter 6 that used patterns, such as the coin-sorting machine example.\n\
  If the value fits the shape of the pattern, we can use the named pieces.\n\
  If it doesn't, the code associated with the pattern won't run.\n\n\
  This chapter is a reference on all things related to patterns.\n\
  We'll cover the valid places to use patterns, the difference between refutable and irrefutable patterns, and the different \
  kinds of pattern syntax that you might see.\n\
  By the end of the chapter, you'll know how to use patterns to express many concepts in a clear way.
  ",
  "x".bright_yellow().bold(),
  "(a, 3)".bright_yellow().bold(),
  "Some(Color::Red)".bright_yellow().bold(),
  "match".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Patterns describe the shape of data.\n\
  {solid_disc} To use a pattern, we compare it to some value.
  ",
  "REMEMBER".bright_yellow().bold(),
  )
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: All the Places Patterns Can Be Used");

  println!(
  "See: {}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html#all-the-places-patterns-can-be-used".bright_cyan()
  )
}













