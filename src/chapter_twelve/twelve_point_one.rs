use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Reading the Argument Values", rtav_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  menu::subheader_title("Section Introduction");

  println!(
    "You can use {0} to pass arguments to the program being run by using two hyphens then a space then argument list which is \
    space seprated.\n\n\
    {1}\n\n\
  ",
    "cargo run".bright_yellow().bold(),
    "cargo run -- <argument1 argument 2 ...>".bright_yellow().bold()
  )
}

// Header: Reading the Argument Values. Abbreviated as rtav.
fn rtav_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Reading the Argument Values");

  println!(
    "To enable a program to read the values of command line arguments we pass to it, we'll need the {0} function \
    provided in Rust's standard library.\n\
    This function returns an iterator of the command line arguments passed to a program.\n\
    For now, you only need to know two details about iterators: Iterators produce a series of values, and we can call \
    the {1} method on an iterator to turn it into a collection, such as a vector, which contains all the elements \
    the iterator produces.\n\n\
    See Listing 12-1: {2}, for code sample\n\n\
    On the first line of main, we call {3}, and we immediately use {1} to turn the iterator into a vector containing \
    all the values produced by the iterator.\n\
    We can use the {1} function to create many kinds of collections, so we explicitly annotate the type of args to \
    specify that we want a vector of strings.\n\
    Although you very rarely need to annotate types in Rust, {1} is one function you do often need to annotate because \
    Rust isn't able to infer the kind of collection you want.
  ",
    "std::env::args".bright_yellow().bold(),
    "collect".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#listing-12-1".cyan(),
    "env::args".bright_yellow().bold(),
  );

  println!(
    "when you run {0}, notice that the first value in the vector is {1}, which is the name of our binary.\n\
    This matches the behavior of the arguments list in C, letting programs use the name by which they were \
    invoked in their execution.\n\
    It’s often convenient to have access to the program name in case you want to print it in messages or change \
    the behavior of the program based on what command line alias was used to invoke the program.
  ",
    "cargo run -- needle haystack".bright_yellow().bold(),
    "\"target/debug/minigrep\"".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    Note that {1} will panic if any argument contains invalid Unicode.\n\
    If your program needs to accept arguments containing invalid Unicode, use {2} instead.\n\
    That function returns an iterator that produces {3} values instead of {4} values.\n\
    We've chosen to use {1} here for simplicity because {3} values differ per platform and are more \
    complex to work with than {4} values.
  ",
    "The args Function and Invalid Unicode".bright_magenta().bold(),
    "std::env::args".bright_yellow().bold(),
    "std::env::args_os".bright_yellow().bold(),
    "OsString".bright_yellow().bold(),
    "String".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} The {1} function creates many kinds of collections, so we need to explicitly annotate \
    because Rust isn't able to infer the kind of collection you want.
  ",
    "REMEMBER".bright_white().bold(),
    "collect".bright_yellow().bold()
  )
}

