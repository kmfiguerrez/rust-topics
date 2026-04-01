use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Writing a Failing Test", waft_content),
    chapter::SubHeader::new("Writing Code to Pass the Test", wctptt_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let two_spaces = "\u{2003}\u{2003}";

  menu::subheader_title("Section Introduction");

  println!(
  "Now that we have the search logic in {0} separate from the main function, it's much easier to write tests for the \
  core functionality of our code.\n\
  {1}.\n\n\
  In this section, we'll add the searching logic to the minigrep program using the test-driven development (TDD) process \
  with the following steps:\n\n\
  {two_spaces}1. Write a test that fails and run it to make sure it fails for the reason you expect.\n\
  {two_spaces}2. Write or modify just enough code to make the new test pass.\n\
  {two_spaces}3. Refactor the code you just added or changed and make sure the tests continue to pass.\n\
  {two_spaces}4. Repeat from step 1!\n\n\
  Though it's just one of many ways to write software, TDD can help drive code design.\n\
  Writing the test before you write the code that makes the test pass helps maintain high test coverage throughout the process.\n\n\
  We'll test-drive the implementation of the functionality that will actually do the searching for the query string in the \
  file contents and produce a list of lines that match the query.\n\
  We'll add this functionality in a function called search.
  ",
  "src/lib.rs".italic(),
  "We can call functions directly with various arguments and check return values without having to call our binary from the \
  command line".bright_white().bold()
  )
}

// Header: Writing a Failing Test. Abbreviated as waft.
fn waft_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Writing a Failing Test");

  println!(
  "In {0}, we'll add a tests module with a test function, as we did in Chapter 11: {1}.\n\
  The test function specifies the behavior we want the search function to have:\n\
  {solid_disc} It will take a query\n\
  {solid_disc} and the text to search\n\
  {solid_disc} and it will return only the lines from the text that contain the query.\n\n\
  See Listing 12-15: {2}, for code sample.\n\n\
  This test searches for the string \"duct\". The text we're searching is three lines, only one of which contains \"duct\" {3} \
  We assert that the value returned from the search function contains only the line we expect.\n\n\
  If we run this test, it will currently fail because the unimplemented! macro panics with the message “not implemented”.\n\
  {4} to get the test to not panic when calling \
  the function by defining the {5} function to always return an empty vector, as shown in Listing 12-16.\n\
  Then, the test should compile and fail because an empty vector doesn't match a vector containing the line {6}.\n\n\
  See Listing 12-16: {7}: Defining just enough of the search function so that calling it won't panic.\n\n\
  Now let's discuss why we need to define an explicit lifetime {8} in the signature of {9} and use that lifetime with the \
  {10} argument and the return value.\n\
  Recall in Chapter 10: {11}, that the lifetime parameters specify which argument lifetime is connected to the lifetime \
  of the return value.\n\
  In this case, we indicate that the returned vector should contain string slices that reference slices of the argument \
  {10} (rather than the argument {12}).\n\n\
  In other words, we tell Rust that the data returned by the {9} function will live as long as the data passed into the \
  {9} function in the {10} argument. This is important!\n\
  The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we're making \
  string slices of {12} rather than {10}, {13}.\n\n\
  See: {14}, for the output errors and read the explanation that follows.
  ",
  "src/lib.rs".italic(),
  "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#the-anatomy-of-a-test-function".cyan(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#listing-12-15".cyan(),
  "(note that the backslash after the opening double quote tells Rust not to put a newline character at \
  the beginning of the contents of this string literal)".bright_white().bold(),
  "In accordance with TDD principles, we'll take a small step of adding just enough code".bright_white().bold(),
  "search".bright_yellow().bold(),
  "\"safe, fast, productive.\"".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#listing-12-16".cyan(),
  "'a".bright_yellow().bold(),
  "search".bright_yellow().bold(),
  "contents".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html".cyan(),
  "query".bright_yellow().bold(),
  "it will do its safety checking incorrectly".bright_red(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#writing-a-failing-test".cyan(),
  )
}

// Header: Writing Code to Pass the Test. Abbreviated as wctptt.
fn wctptt_content() {
  let two_spaces = "\u{2003}\u{2003}";

  menu::subheader_title("Writing Code to Pass the Test");

  println!(
  "Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to \
  follow these steps:\n\
  {two_spaces}1. Iterate through each line of the contents.\n\
  {two_spaces}2. Check whether the line contains our query string.\n\
  {two_spaces}3. If it does, add it to the list of values we're returning.\n\
  {two_spaces}4. If it doesn't, do nothing.\n\
  {two_spaces}5. Return the list of results that match.\n\n\
  Let's work through each step, starting with iterating through lines.
  ");

  println!(
  "{0}\n\n\
  Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines, that works as shown in \
  Listing 12-17.\n\
  Note that this won't compile yet.\n\n\
  See Listing 12-17: {1}, for code sample.\n\n\
  The {2} method returns an iterator.\n\
  We can use a {3} loop with an iterator to run some code on each item in a collection.
  ",
  "Iterating Through Lines with the lines Method".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#listing-12-17".cyan(),
  "lines".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Next, we'll check whether the current line contains our query string.\n\
  Fortunately, strings have a helpful method named {1} that does this for us!\n\
  Add a call to the {1} method in the search function, as shown in Listing 12-18.\n\
  Note that this still won't compile yet.\n\n\
  See: Listing 12-18: {2}, for code sample.\n\n\
  At the moment, we're building up functionality.\n\
  To get the code to compile, we need to return a value from the body as we indicated we would in the function signature.
  ",
  "Searching Each Line for the Query".bright_magenta().bold(),
  "contains".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#listing-12-18".cyan()
  );

  println!(
  "{0}\n\n\
  To finish this function, we need a way to store the matching lines that we want to return.\n\
  For that, we can make a mutable vector before the {1} loop and call the {2} method to store a {3} in the vector.\n\
  After the for loop, we return the vector, as shown in Listing 12-19.\n\n\
  See: Listing 12-19: {4}, for code sample.\n\n\
  Now the search function should return only the lines that contain query, and our test should pass.\n\
  Let's run the test:\n\n\
  See: {5}, for the output of the test and read the explanation that follows.
  ",
  "Storing Matching Lines".bright_magenta().bold(),
  "for".bright_yellow().bold(),
  "push".bright_yellow().bold(),
  "line".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#listing-12-19".cyan(),
  "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#storing-matching-lines".cyan(),
  );  
}

