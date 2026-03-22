use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", chtar_content),
    chapter::SubHeader::new("Running Tests in Parallel or Consecutively", rtipoc_content),
    chapter::SubHeader::new("Showing Function Output", sfo_content),
    chapter::SubHeader::new("Running a Subset of Tests by Name", rasotbn_content),
    chapter::SubHeader::new("Ignoring Tests Unless Specifically Requested", itusr_content),


  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}


// Subheaders content below.

// Header: Controlling How Tests Are Run. Abbreviated as chtar.
fn chtar_content() {
  menu::subheader_title("Section: Controlling How Tests Are Run, Introduction");

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

// Header: Showing Function Output. Abbreviated as sfo.
fn sfo_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Running Tests in Parallel or Consecutively");

  println!(
    "By default, if a test passes, Rust's test library captures anything printed to standard output.\n\
    For example, if we call {0} in a test and the test passes, we won't see the {0} output in the terminal; \
    we'll see only the line that indicates the test passed.\n\
    If a test fails, we'll see whatever was printed to standard output with the rest of the failure message.\n\n\
    As an example, Listing 11-10 has a silly function that prints the value of its parameter and returns 10, as well \
    as a test that passes and a test that fails.\n\n\
    See: {1}, for code sample.\n\n\
    Note that nowhere in this output do we see {2}, which is printed when the test that passes runs.\n\
    That output has been captured.\n\
    The output from the test that failed, {3}, appears in the section of the test summary output, which also shows the cause \
    of the test failure.\n\n\
    If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests \
    with {4}:\n\n\
    {5}
  ",
    "println!".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#listing-11-10".cyan(),
    "I got the value 4".bright_yellow().bold(),
    "I got the value 8".bright_yellow().bold(),
    "--show-output".bright_yellow().bold(),
    "$ cargo test -- --show-output".bright_yellow().bold()

  );

  println!(
    "{}\n\n\
    {solid_disc} By default, if a test passes, Rust's test library captures anything printed to standard output.
  ",
    "REMEMBER".bright_white().bold(),
  )  
}

// Header: Running a Subset of Tests by Name. Abbreviated as rasotbn.
fn rasotbn_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Running a Subset of Tests by Name");

  println!(
    "Running a full test suite can sometimes take a long time.\n\
    If you're working on code in a particular area, you might want to run only the tests pertaining to that code.\n\
    You can choose which tests to run by passing {0} the name or names of the test(s) you want to run as an argument.\n\n\
    To demonstrate how to run a subset of tests, we'll first create three tests for our add_two function, as shown in 
    Listing 11-11, and choose which ones to run.\n\n\
    See: {1}, for code sample and outputs.
  ",
    "cargo test".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#listing-11-11".cyan()
  );

  println!(
    "{0}\n\n\
    We can pass the name of any test function to {1} to run only that test:\n\n\
    {1}\n\n\
    Only the test with the name one_hundred ran; the other two tests didn't match that name.\n\
    The test output lets us know we had more tests that didn't run by displaying {3} at the end.\n\
    We can't specify the names of multiple tests in this way; only the first value given to {2} will be used. \
    But there is a way to run multiple tests.
  ",
    "Running Single Tests".bright_magenta().bold(),
    "cargo test <test name>".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
    "2 filtered out".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    We can specify part of a test name, and any test whose name matches that value will be run.\n\
    For example, because two of our tests' names contain add, we can run those two by running {1}:\n\n\
    {2}\n\n\
    This command ran all tests with {3} in the name and filtered out the tests that don't.\n\n\
    Also note that the module in which a test appears becomes part of the test's name, so we can run all \
    the tests in a module by filtering on the module's name.
  ",
    "Filtering to Run Multiple Tests".bright_magenta().bold(),
    "cargo test add".bright_yellow().bold(),
    "cargo test <part of a test name>".bright_yellow().bold(),
    "add".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} Also note that the module in which a test appears becomes part of the test's name, so we can run all the tests \
    in a module by filtering on the module's name.
  ",
    "REMEMBER".bright_white().bold(),
  )    

}

// Header: Ignoring Tests Unless Specifically Requested. Abbreviated as itusr.
fn itusr_content() {
  menu::subheader_title("Ignoring Tests Unless Specifically Requested");

  println!(
    "Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of \
    {0}.\n\
    Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the \
    {1} attribute to exclude them.\n\n\
    See: {2}, for codes sample and output.\n\n\
    After {3}, we add the {4} line to the test we want to exclude.\n\
    Now when we run our tests, {5} runs, but {6} doesn't.\n\
    The {6} function is listed as ignored.\n\
    If we want to run only the ignored tests, we can use {7}.\n\n\
    By controlling which tests run, you can make sure your {0} results will be returned quickly.\n\
    When you're at a point where it makes sense to check the results of the ignored tests and you have time to wait for the \
    results, you can run {7} instead.\n\
    If you want to run all tests whether they're ignored or not, you can run {8}.

  ",
    "cargo test".bright_yellow().bold(),
    "ignore".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested".cyan(),
    "#[test]".bright_yellow().bold(),
    "#[ignore]".bright_yellow().bold(),
    "it_works".bright_yellow().bold(),
    "expensive_test".bright_yellow().bold(),
    "cargo test -- --ignored".bright_yellow().bold(),
    "cargo test -- --include-ignored".bright_yellow().bold(),
  )
}


