use owo_colors::OwoColorize;
use crate::{chapter, menu};


pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 8];
  subheaders = [
    chapter::SubHeader::new("Introduction", introduction_content),
    chapter::SubHeader::new("How to Write Tests", htwt_content),
    chapter::SubHeader::new("Structuring Test Functions", stf_content),
    chapter::SubHeader::new("Checking Results with assert!", crwa_content),
    chapter::SubHeader::new("Testing Equality with assert_eq! and assert_ne!", tewegane_content),
    chapter::SubHeader::new("Adding Custom Failure Messages", acfm_content),
    chapter::SubHeader::new("Checking for Panics with should_panic", cfpws_content),
    chapter::SubHeader::new("Using Result<T, E> in Tests", urit_content),

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
    "This part talks about bug that can be caught with code testing.\n\n\
    The {0} macro, provided by the standard library, is useful when you want to ensure that some condition in a \
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
  );

  println!(
    "Note the use {0} line inside the {1} module.\n\
    The {1} module is a regular module that follows the usual visibility rules: {2}\n\
    Because the {1} module is an inner module, we need to bring the code under test (code under test means code being used \
    in the test functions) in the outer module into the scope of the inner module.\n\
    The glob (*) in the super means anything we define in the outer module is available to this tests module.
  ",
    "super::*;".bright_yellow().bold(),
    "tests".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html".cyan()
  );

  println!(
    "The rest of the code samples in the Checking Results with assert! subheader talks about different scenarios .i.e. \
    what happens the smaller rectangle gets passed with larger rectangle and catching a bug when you flip the greater-than \
    sign to > with the < less that sign.
  ");

}

// Header: Testing Equality with assert_eq! and assert_ne!. Abbreviated as teweqane.
fn tewegane_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Testing Equality with assert_eq! and assert_ne!");
  
  println!(
    "A common way to verify functionality is to test for equality between the result of the code under test and \
    the value you expect the code to return. \n\
    You could do this by using the {0} macro and passing it an expression using the {1} operator.\n\
    However, this is such a common test that the standard library provides a pair of macros—{2} and {3}—to perform \
    this test more conveniently.\n\
    These macros compare two arguments for equality or inequality, respectively.\n\
    They'll also print the two values if the assertion fails, which makes it easier to see why the test failed; \
    conversely, the {0} macro only indicates that it got a {4} value for the {1} expression, without printing \
    the values that led to the {4} value.\n\n\
    See Listing 11-7:{5}, for code sample
  ",
    "assert!".bright_yellow().bold(),
    "==".bright_yellow().bold(),
    "assert_eq!".bright_yellow().bold(),
    "assert_ne!".bright_yellow().bold(),
    "false".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-7".cyan()
  );

  println!(
    "In the part where the code is modified to introduce a bug, you'll see a message that the assertion \
    that failed was {0} and what the {1} and {2} values are.\n\
    This message helps us start debugging!\n\
    You can imagine that this would be especially helpful when we have a lot of tests going on.
  ",
    "left == right".bright_yellow().bold(),
    "left".bright_yellow().bold(),
    "right".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    Note that in some languages and test frameworks, the parameters to equality assertion functions are called {1} and {2}, \
    and the order in which we specify the arguments matters.\n\
    However, in Rust, they're called {3} and {4}, and the order in which we specify the value we expect and the value the \
    code produces doesn't matter.
  ",
    "TESTING IN OTHER LANGUAGE AND FRAMEWORKS".bright_magenta().bold(),
    "expected".bright_yellow().bold(),
    "actual".bright_yellow().bold(),
    "left".bright_yellow().bold(),
    "right".bright_yellow().bold(),
  );

  println!(
    "The {0} macro will pass if the two values we give it are not equal and will fail if they are equal.\n\
    {1}.\n\
    For example, if we're testing a function that is guaranteed to change its input in some way, but the \
    way in which the input is changed depends on the day of the week that we run our tests, the best thing \
    to assert might be that the output of the function is not equal to the input.
  ",
    "assert_ne!".bright_yellow().bold(),
    "This macro is most useful for cases when we're not sure what a value will be, but we know what the value \
    definitely shouldn't be".bright_white().bold()
  );

  println!(
    "Under the surface, the {0} and {1} macros use the operators {2} and {3}, respectively.\n\
    When the assertions fail, these macros print their arguments using debug formatting, which \
    means the values being compared must implement the {4} and {5} traits.\n\
    All primitive types and most of the standard library types implement these traits.\n\
    For structs and enums that you define yourself, you'll need to implement {4} to assert equality of those types.\n\
    You'll also need to implement {5} to print the values when the assertion fails.\n\
    Because both traits are derivable traits, as mentioned in Listing 5-12: {6} in Chapter 5, this is usually as straightforward \
    as adding the {7} annotation to your struct or enum definition.
  ",
    "assert_eq!".bright_yellow().bold(),
    "assert_ne!".bright_yellow().bold(),
    "==".bright_yellow().bold(),
    "!=".bright_yellow().bold(),
    "PartialEq".bright_yellow().bold(),
    "Debug".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch05-02-example-structs.html#listing-5-12".cyan(),
    "#[derive(PartialEq, Debug)]".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    {solid_disc} In other languages and frameworks, the order of the arguments you pass to {1} and {2} matters. \
    But in Rust the order of the arguments doesn't matter.\n\
    {solid_disc} Under the surface, the {1} and {2} macros use the operators {3} and {4}, respectively, \
    which means the values being compared must implement the {5} and {6} traits.\n\
    {solid_disc} For structs and enums that you define yourself, you'll need to implement {5} to assert equality of those types.\n\
    {solid_disc} You'll also need to implement {6} to print the values when the assertion fails.

  ",
    "REMEMBER".bright_white().bold(),
    "assert_eq!".bright_yellow().bold(),
    "assert_ne!".bright_yellow().bold(),
    "==".bright_yellow().bold(),
    "!=".bright_yellow().bold(),
    "PartialEq".bright_yellow().bold(),
    "Debug".bright_yellow().bold(),    
  )
}

// Header: Adding Custom Failure Messages. Abbreviated as acfm.
fn acfm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Adding Custom Failure Messages");
  
  println!(
    "You can also add a custom message to be printed with the failure message as optional arguments to the \
    {0}, {1}, and {2} macros.\n\
    Any arguments specified after the required arguments are passed along to the {3} macro \
    (discussed in “Concatenating with + or format!”: {4} in Chapter 8), so you can pass a format string that contains {{}} \
    placeholders and values to go in those placeholders.\n\
    Custom messages are useful for documenting what an assertion means; when a test fails, you'll have a better idea of what \
    the problem is with the code.\n\n\
    See: {5}, for code samples and explanations.
  ",
    "assert!".bright_yellow().bold(),
    "assert_eq!".bright_yellow().bold(),
    "assert_ne!".bright_yellow().bold(),
    "format!".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch08-02-strings.html#concatenating-with--or-format".cyan(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#adding-custom-failure-messages".cyan()
  );

  println!(
    "{0}\n\n\
    {solid_disc} Any arguments passed after the required arguments to assert macros are passed along to the {1} macro, \
    meaning values can be be specified directly inside {{}}.
  ",
    "REMEMBER".bright_white().bold(),
    "format!".bright_yellow().bold(),

  )
}

// Header: Checking for Panics with should_panic. Abrreviated as cfpws.
fn cfpws_content() {
  menu::subheader_title("Checking for Panics with should_panic");

  println!(
    "In addition to checking return values, it's important to check that our code handles error conditions as we expect.\n\
    For example, consider the Guess type that we created in Chapter 9, Listing 9-13: {0}.\n\
    Other code that uses {1} depends on the guarantee that {1} instances will contain only values between 1 and 100.\n\
    We can write a test that ensures that attempting to create a Guess instance with a value outside that range panics.\n\n\
    We do this by adding the attribute {2} to our test function.\n\
    The test passes if the code inside the function panics; the test fails if the code inside the function doesn't panic.\n\
    Listing 11-8 shows a test that checks that the error conditions of Guess::new happen when we expect them to.\n\n\
    Listing 11-8: {3}, for code sample.\n\n\
    We place the {4} attribute after the {5} attribute and before the test function it applies to.
  ",
    "https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#listing-9-13".cyan(),
    "Guess".bright_yellow().bold(),
    "should_panic".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-8".cyan(),
    "#[should_panic]".bright_yellow().bold(),
    "#[test]".bright_yellow().bold(),
  );

  println!(
    "See: {0}, and look for the first codes that introduces bugs and explanations.\n\n\
    Tests that use {1} can be imprecise.Test.\n\
    A {1} test would pass even if the test panics for a different reason from the one we were expecting.\n\
    To make {1} tests more precise, we can add an optional {2} parameter to the {1} attribute.\n\
    The test harness will make sure that the failure message contains the provided text.\n\
    For example, consider the modified code for {3} in Listing 11-9 where the {4} function panics with \
    different messages depending on whether the value is too small or too large.\n\n\
    Listing 11-9: {5}\n\n\
    This test will pass because the value we put in the {1} attribute's expected parameter is a \
    substring of the message that the {6} function panics with.\n\
    We could have specified the entire panic message that we expect, which in this case would be {7}.\n\
    What you choose to specify depends on how much of the panic message is unique or dynamic and how precise \
    you want your test to be.\n\
    In this case, a substring of the panic message is enough to ensure that the code in the test function executes \
    the {8} case.\n\n\
    The next reading introduces another bug.\n\
    Again, see: {0}, for code sample and explanation in the bottom.
  ",
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic".cyan(),
    "should_panic".bright_yellow().bold(),
    "expected".bright_yellow().bold(),
    "Guess".bright_yellow().bold(),
    "new".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#listing-11-9".cyan(),
    "Guess::new".bright_yellow().bold(),
    "Guess value must be less than or equal to 100, got 200".bright_yellow().bold(),
    "else if value > 100".bright_yellow().bold(),
  )
}

// Header: Using Result<T, E> in Tests. Abbreviated as urit.
fn urit_content() {
  menu::subheader_title("Using Result<T, E> in Tests");

  println!(
    "We can also write tests that use {0}\n\n\
    See: {1}, for code sample.\n\n\
    The {2} function now has the {3} return type.\n\
    In the body of the function, rather than calling the {4} macro, we return {5} when the test passes and an {6} with \
    a {7} inside when the test fails.
  ",
    "Result<T, E>!".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests".cyan(),
    "it_works".bright_yellow().bold(),
    "Result<(), String>".bright_yellow().bold(),
    "assert_eq!".bright_yellow().bold(),
    "Ok(())".bright_yellow().bold(),
    "Err".bright_yellow().bold(),
    "String".bright_yellow().bold(),
  );

  println!(
    "Writing tests so that they return a {0} enables you to use the question mark operator in the body of tests, which can \
    be a convenient way to write tests that should fail if any operation within them returns an {1} variant.\n\n\
    You can't use the {2} annotation on tests that use {0}.\n\
    To assert that an operation returns an {1} variant, don't use the question mark operator on the {0} value.\n\
    Instead, use {3}.
  ",
    "Result<T, E>".bright_yellow().bold(),
    "Err".bright_yellow().bold(),
    "#[should_panic]".bright_yellow().bold(),
    "assert!(value.is_err())".bright_yellow().bold(),
  )
}