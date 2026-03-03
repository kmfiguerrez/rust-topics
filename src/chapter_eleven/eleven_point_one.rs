use owo_colors::OwoColorize;
use crate::{chapter, menu};


pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Introduction", introduction_content),
    chapter::SubHeader::new("How to Write Tests", htwt_content),
    chapter::SubHeader::new("Structuring Test Functions", stf_content),
    chapter::SubHeader::new("Checking Results with assert!", crwa_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  menu::subheader_title("Introduction");

  println!(
    "In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that “program testing can be a very effective way to show the presence of bugs, \
    but it is hopelessly inadequate for showing their absence.” \n\
    That doesn't mean we shouldn't try to test as much as we can!
  ");

  println!(
    "{0} in our programs is the extent to which our code does what we intend it to do.\n\
    Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.\n\
    Rust's type system shoulders a huge part of this burden, but the type system cannot catch everything.\n\
    As such, Rust includes support for writing automated software tests.
  ",
    "Correctness".italic()
  );

  println!(
    "Say we write a function {0} that adds 2 to whatever number is passed to it.\n\
    This function's signature accepts an integer as a parameter and returns an integer as a result.\n\
    When we implement and compile that function, Rust does all the type checking and borrow checking that you've learned so far to ensure that, for instance, \
    we aren't passing a {1} value or an invalid reference to this function.\n\
    But Rust can't check that this function will do precisely what we intend, which is return the parameter plus 2 rather than, say, the parameter plus 10 or \
    the parameter minus 50! That's where tests come in.
  ",
    "add_two".bright_yellow().bold(),
    "String".bright_yellow().bold(),
  );

  println!(
    "We can write tests that assert, for example, that when we pass {0} to the {1} function, the returned value is {2}.\n\
    We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.
  ",
    "3".bright_yellow().bold(),
    "add_two".bright_yellow().bold(),
    "5".bright_yellow().bold(),
   )
}

// Header: How to Write Tests. Abrreviated as htwt.
fn htwt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("How to Write Tests");

  println!(
    "{0} are Rust functions that verify that the non-test code is functioning in the expected manner.\n\
    The bodies of test functions typically perform these three actions:\n\n\
    {solid_disc} Set up any needed data or state.\n\
    {solid_disc} Run the code you want to test.\n\
    {solid_disc} Assert that the results are what you expect.\n\n\
    Let's look at the features Rust provides specifically for writing tests that take these actions, which include the {1} attribute, \
    a few macros, and the {2} attribute.
  ",
    "Tests".italic(),
    "Tests".bright_yellow().bold(),
    "should_panic".bright_yellow().bold(),
  )
}

// Header: Structuring Test Functions. Abbreviated as stf.
fn stf_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Structuring Test Functions");

  println!(
    "At its simplest, {0} that's annotated with the {1} attribute.\n\
    {2}; one example is the {3} attribute we used with structs.\n\
    To change a function into a test function, add {4} on the line before {5}.\n\
    When you run your tests with the {6} command, Rust builds a test runner binary that runs the annotated functions and reports \
    on whether each test function passes or fails.
  ",
    "a test in Rust is a function".bright_white().bold(),
    "test".bright_yellow().bold(),
    "Attributes are metadata about pieces of Rust code".bright_white().bold(),
    "derive".bright_yellow().bold(),
    "#[test]".bright_yellow().bold(),
    "fn".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
  );

  println!(
    "Whenever we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.\n\
    This module gives you a template for writing your tests so that you don't have to look up the exact structure and syntax every \
    time you start a new project.\n\
    You can add as many additional test functions and as many test modules as you want!
  ");

  println!(
    "See: {}, for code sample\n\n\
    We might also have non-test functions in the {1} module to help set up common scenarios or perform common operations, \
    so we always need to indicate which functions are tests.\n\
    The {1} function body uses the {2} macro to assert that result, which contains the result of calling {3} with 2 and 2, equals 4.\n\
    This assertion serves as an example of the format for a typical test.\n\
    Let's run it to see that this test passes.\n\n\
    See: {4}, for the ouput of running the {5}.\n\n\
    In the result ouput, you'll see the name of the generated test function, called {6} and the result of running that test is {7} on the same line.\n\
    The overall summary {8}. means that all the tests passed, and the portion that reads {9} totals the number of tests that passed or failed.
  ",
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-1".cyan(),
    "test".bright_yellow().bold(),
    "assert_eq!".bright_yellow().bold(),
    "add".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-2".cyan(),
    "cargo test".bright_yellow().bold(),
    "tests::it_works".bright_yellow().bold(),
    "ok".bright_yellow().bold(),
    "test result: ok".bright_yellow().bold(),
    "1 passed; 0 failed".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    It's possible to mark a test as ignored so that it doesnt run in a particular instance; it will show a count in the \
    {1} part in the summary line. \n\
    See: {2}\n\n\
    {3}\n\n\
    We can also pass an argument to the {4} command to run only tests whose name matches a string; this is called {5}, \
    the summary line will show a count in the {6} part.\n\
    See: {7}
  ",
    "IGNORING TESTS".bright_magenta().bold(),
    "ignored".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested".cyan(),
    "FILTERING OUT TESTS".bright_magenta().bold(),
    "cargo test".bright_yellow().bold(),
    "filtering".italic(),
    "filtered out".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name".cyan()
  );

  println!(
    "The next part of the test output starting at {0} adder is for the results of any documentation tests.\n\
    The code sample above doesn't show any documentation tests yet, but Rust can compile any code examples that appear in our API documentation.\n\
    This feature helps keep your docs and your code in sync! See: {1}
  ",
    "Doc-tests".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests".cyan()
  );

  println!(
    "{0}\n\n\
    Now we'll add another test, but this time we'll make a test that fails!\n\
    Tests fail when something in the test function panics.\n\
    Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.\n\n\
    See: {1}, for code sample\n\n\
    See: {2}, for test results\n\n\
    Two new sections appear between the individual results and the summary:\n\
    {solid_disc} The first displays the detailed reason for each test failure. \
    In this case, we get the details that {3} failed because it panicked with the message {4} on line 17 in the {5}.\n\
    {solid_disc} The next section lists just the names of all the failing tests, which is useful when there are lots \
    of tests and lots of detailed failing test output. We can use the name of a failing test to run just that test to debug it more easily; \
    See: {6}, for more info.\n\n\
    The summary line displays at the end: Overall, our test result is {7}.\n\
    We had one test pass and one test fail.
  ",
    "TEST THAT FAILS".bright_magenta().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-3".cyan(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-4".cyan(),
    "tests::another".bright_yellow().bold(),
    "Make this test fail".bright_yellow().bold(),
    "src/lib.rs file".italic(),
    "https://doc.rust-lang.org/book/ch11-02-running-tests.html#controlling-how-tests-are-run".cyan(),
    "FAILED".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} A test in Rust is a function that's annotated with the test attribute.\n\
    {solid_disc} Attributes are metadata about pieces of Rust code.\n\
    {solid_disc} Rust builds a test runner binary that runs the annotated functions and reports the result.\n\
    {solid_disc} You can add as many additional test functions and as many test modules as you want!.\n\
    {solid_disc} We might also have non-test functions in the {1} module to help set up common scenarios or perform common operations.\n\
    {solid_disc} Tests can be ignored and filtered out.\n\
    {solid_disc} Tests fail when something in the test function panics.\n\
  ",
    "REMEMBER".bright_white().bold(),
    "test".bright_yellow().bold(),
  )

}

// Header: Checking Results with assert!. Abbreviated as crwa.
fn crwa_content() {
  menu::subheader_title("Checking Results with assert!");

  println!(
    "The {0} macro, provided by the standard library, is useful when you want to ensure that some condition in a \
    test evaluates to {1}.\n\
    We give the {0} macro an argument that evaluates to a Boolean.\n\
    If the value is {1}, nothing happens and the test passes.\n\
    If the value is {2}, the assert! macro calls {3} to cause the test to fail.\n\
    Using the {0} macro helps us check that our code is functioning in the way we intend.\n\n\
    See Listing 11-5 for the {4} struct definition. and Listing 11-6 for test code sample.\n\n\
    Listing 11-5: {5}\n\
    Listing 11-6: {6}\n\
  ",
    "assert!".bright_yellow().bold(),
    "true".bright_yellow().bold(),
    "false".bright_yellow().bold(),
    "panic!".bright_yellow().bold(),
    "Rectangle".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-5".cyan(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-6".cyan(),
  );

  println!(
    "The {0} method returns a Boolean, which means it's a perfect use case for the {1} macro.\n\
    In Listing 11-6, we write a test that exercises the {0} method by creating a large {2} instance \
    and asserting that it can hold a smaller instance of {2}.
  ",
    "can_hold".bright_yellow().bold(),
    "assert!".bright_yellow().bold(),
    "Rectangle".bright_yellow().bold(),
  )

}


