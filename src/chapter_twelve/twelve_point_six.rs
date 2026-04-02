use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Checking Where Errors Are Written", cweaw_content),
    chapter::SubHeader::new("Printing Errors to Standard Error", petse_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction");

  println!(
  "At the moment, we're writing all of our output to the terminal using the {0} macro.\n\
  In most terminals, there are two kinds of output:\n\
  {solid_disc} standard output ({1}) for general information\n\
  {solid_disc} standard error ({2}) for error messages\n\n\
  This distinction enables users to choose to direct the successful output of a program to a file but still print \
  error messages to the screen.\n\n\
  The {0} macro is only capable of printing to standard output, so we have to use something else to print to standard error.
  ",
  "println!".bright_yellow().bold(),
  "stdout".bright_cyan().bold(),
  "stderr".bright_cyan().bold(),
  )
}

// Header: Checking Where Errors Are Written. Abbreviated as cweaw.
fn cweaw_content() {
  menu::subheader_title("Checking Where Errors Are Written");

  println!(
  "First, let's observe how the content printed by {0} is currently being written to standard output, \
  including any error messages we want to write to standard error instead.\n\
  We'll do that by redirecting the standard output stream to a file while intentionally causing an error.\n\
  We won't redirect the standard error stream, so any content sent to standard error will continue to display on the screen.\n\n\
  Command line programs are expected to send error messages to the standard error stream so that we can still see error messages \
  on the screen even if we redirect the standard output stream to a file.\n\
  Our program is not currently well behaved: We're about to see that it saves the error message output to a file instead!\n\n\
  To demonstrate this behavior, we'll run the program with {1} and the file path, {2}, that we want to redirect the standard output \
  stream to.\n\
  We won't pass any arguments, which should cause an error:\n\n\
  {3}\n\n\
  The {1} syntax tells the shell to write the contents of standard output to {2} instead of the screen.\n\
  We didn't see the error message we were expecting printed to the screen, so that means it must have ended up in the file.\n\
  This is what {2} contains:
  {4}\n\n\
  Yup, our error message is being printed to standard output.\n\
  It's much more useful for error messages like this to be printed to standard error so that only data from a successful run ends \
  up in the file.\n\
  We'll change that.
  ",
  "println!".bright_yellow().bold(),
  ">".bright_yellow().bold(),
  "output.txt".italic(),
  "cargo run > output.txt".bright_yellow().bold(),
  "Problem parsing arguments: not enough arguments".bright_yellow().bold(),
  )
}

// Header: Printing Errors to Standard Error. Abbreviated as petse.
fn petse_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Printing Errors to Standard Error");

  println!(
  "We'll use the code in Listing 12-24 to change how error messages are printed.\n\
  Because of the refactoring we did earlier in this chapter, all the code that prints error messages is in one function, {0}.\n\
  The standard library provides the {1} macro that prints to the standard error stream, so let's change the two places we \
  were calling {1} to print errors to use {1} instead.\n\n\
  See Listing 12-24: {2}, for code sample, explanation and code outputs.\n\n\
  Let's now run the program again in the same way, without any arguments and redirecting standard output with {3}:\n\n\
  {4}\n\n\
  Now we see the error onscreen and output.txt contains nothing, which is the behavior we expect of command line programs.\n\n\
  Let's run the program again with arguments that don't cause an error but still redirect standard output to a file, like so:\n\n\
  {5}\n\n\
  We won't see any output to the terminal, and output.txt will contain our results:\n\n\
  Filename: output.txt\n\n\
  {6}\n\n\
  This demonstrates that were now using standard output for successful output and standard error for error output as appropriate.
  ",
  "main".bright_yellow().bold(),
  "eprintln!".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html#listing-12-24".bright_yellow().bold(),
  ">".bright_yellow().bold(),
  "cargo run > output.txt\n\
  Problem parsing arguments: not enough arguments".bright_yellow().bold(),
  "cargo run -- to poem.txt > output.txt".bright_yellow().bold(),
  "Are you nobody, too?\n\
  How dreary to be somebody!".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Both the standard output and standard error streams are still being printed to the terminal by default, \
  so if we don't redirect either of them, we'll see all output on the screen.\n\
  {solid_disc} By fault the {2} operator redirects standard output only.\n\
  {solid_disc} If we redirect standard output, we'll only see error messages on the screen (using {1}) and the successful output \
  will go to the file.\n\
  {solid_disc} If we redirect standard error, we'll only see successful output on the screen and the \
  error messages will go to the file.\n\
  {solid_disc} If we redirect both standard output and standard error, we won't see any output on the screen and all \
  output will go to the file.
  {solid_disc} Good refactoring means keeping the error handling logic in one place - \
  like codes that print error messages are in {3} function.
  ",
  "REMEMBER".bright_white().bold(),
  "eprintln!".bright_yellow().bold(),
  ">".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  )
}
