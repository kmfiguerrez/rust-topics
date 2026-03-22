use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Controlling How Tests Are Run", chtar_content),
    chapter::SubHeader::new("Running Tests in Parallel or Consecutively", rtipoc_content),


  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}


// Subheaders content below.

// Header: Controlling How Tests Are Run. Abbreviated as chtar.
fn chtar_content() {
  menu::subheader_title("Controlling How Tests Are Run");

  println!(
    "Just as {0} compiles your code and then runs the resultant binary, {1} compiles your code in test mode and runs \
    the resultant test binary.\n\
    The default behavior of the binary produced by {1} is to run all the tests in parallel and capture output generated \
    during test runs, preventing the output from being displayed and making it easier to read the output related to the \
    test results.\n\
    You can, however, specify command line options to change this default behavior.\n\n\
    Some command line options go to {1}, and some go to the resultant test binary.\n\
    To separate these two types of arguments, you list the arguments that go to {1} followed by the separator {2} and then \
    the ones that go to the test binary.\n\
    Running {3} displays the options you can use with {1}, and running {4} displays the options you can use after the separator.
  ",
    "cargo run".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
    "--".bright_yellow().bold(),
    "cargo test --help".bright_yellow().bold(),
    "cargo test -- --help".bright_yellow().bold(),
  );
}

// Header: Running Tests in Parallel or Consecutively. Abbreviated as rtipoc.
fn rtipoc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Running Tests in Parallel or Consecutively");

  println!(
    "When you run multiple tests, by default they run in parallel using threads, meaning they finish running more quickly \
    and you get feedback sooner.\n\
    Because the tests are running at the same time, you must make sure your tests don't depend on each other or on any shared \
    state, including a shared environment, such as the current working directory or environment variables.\n\n\
    For example, say each of your tests runs some code that creates a file on disk named {0} and writes some data \
    to that file.\n\
    Then, each test reads the data in that file and asserts that the file contains a particular value, which is \
    different in each test.\n\
    Because the tests run at the same time, one test might overwrite the file in the time between when another test is \
    writing and reading the file.\n\
    The second test will then fail, not because the code is incorrect but because the tests have interfered with each other \
    while running in parallel.\n\
    {1}\n\n\
    If you don't want to run the tests in parallel or if you want more fine-grained control over the number of threads used, \
    you can send the {2} flag and the number of threads you want to use to the test binary.\n\
    Take a look at the following example:\n\n\
    {3}\n\n\
    We set the number of test threads to 1, telling the program not to use any parallelism.\n\
    Running the tests using one thread will take longer than running them in parallel, \
    but the tests won't interfere with each other if they share state.
  ",
    "test-output.txt".italic(),
    "One solution is to make sure each test writes to a different file; \
    another solution is to run the tests one at a time.".bright_white().bold(),
    "--test-threads".bright_yellow().bold(),
    "$ cargo test -- --test-threads=1".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    {solid_disc} When you run multiple tests, by default they run in parallel using threads, meaning they finish running more quickly \
    and you get feedback sooner.\n\
    {solid_disc} Because the tests are running at the same time, you must make sure your tests don't depend on each other or on any \
    shared state, including a shared environment, such as the current working directory or environment variables.\n\
    {solid_disc} If you don't want to run the tests in parallel or if you want more fine-grained control over the number of threads \
    used, you can send the {1} flag.\n\
    {solid_disc} When we set the number of test threads to 1, telling the program not to use any parallelism.
  ",
    "REMEMBER".bright_white().bold(),
    "--test-threads".bright_yellow().bold()
  )

}