use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", to_content),
    chapter::SubHeader::new("Unit Tests", ut_content),
    chapter::SubHeader::new("Integration Tests", it_content),
    chapter::SubHeader::new("Integration Tests", s_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Test Organization. Abbreviated as to.
fn to_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section: Test Organization, introduction");

  println!(
    "As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and \
    organization.\n\
    The Rust community thinks about tests in terms of two main categories:\n\n\
    {solid_disc} unit tests.\n\
    {solid_disc} integration tests.\n\n\
    {0} are small and more focused, testing one module in isolation at a time, and can test private interfaces.\n\
    {1} are entirely external to your library and use your code in the same way any other external code would, \
    using only the public interface and potentially exercising multiple modules per test.\n\n\
    Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to, \
    separately and together.
  ",
    "Unit tests".italic().bold(),
    "Integration tests".italic().bold()
  )
}

// Header: Unit Tests. Abbreviated as ut.
fn ut_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Unit Tests");

  println!(
    "The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code \
    is and isn't working as expected.\n\
    You'll put unit tests in the src directory in each file with the code that they're testing.\n\
    The convention is to create a module named {0} in each file to contain the test functions and to annotate the module with \
    {1}.
  ",
    "tests".bright_yellow().bold(),
    "cfg(test)".bright_yellow().bold()
  );

  println!(
    "{0}\n\n\
    The {1} annotation on the {2} module tells Rust to compile and run the test code only when you run {3}, \
    not when you run {4}.\n\
    This saves compile time when you only want to build the library and saves space in the resultant compiled artifact \
    because the tests are not included.\n\
    You'll see that because integration tests go in a different directory, they don't need the {1} annotation.\n\
    However, because unit tests go in the same files as the code, you'll use {1} to specify that they shouldn't be \
    included in the compiled result.\n\n\
    On the automatically generated {2} module, the attribute {5} stands for {6} and tells Rust that the \
    following item should only be included given a certain configuration option.\n\
    In this case, the configuration option is test, which is provided by Rust for compiling and running tests.\n\
    By using the {5} attribute, Cargo compiles our test code only if we actively run the tests with {3}.\n\
    This includes any helper functions that might be within this module, in addition to the functions annotated with {7}.
  ",
    "The tests Module and #[cfg(test)]".bright_magenta().bold(),
    "#[cfg(test)]".bright_yellow().bold(),
    "tests".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
    "cargo build".bright_yellow().bold(),
    "cfg".bright_yellow().bold(),
    "configuration".italic().bold(),
    "#[test]".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    There's debate within the testing community about whether or not private functions should be tested directly, and other \
    languages make it difficult or impossible to test private functions.\n\
    Regardless of which testing ideology you adhere to, Rust's privacy rules do allow you to test private functions.\n\
    Consider the code in Listing 11-12 with the private function {1}.\n\n\
    See: {2}, for code sample.\n\n\
    Note that the {1} function is not marked as {3}.\n\
    Tests are just Rust code, and the {4} module is just another module.\n\
    As we discussed in “Paths for Referring to an Item in the Module Tree”: {5}, items in child modules can use \
    the items in their ancestor modules.\n\
    In this test, we bring all of the items belonging to the {4} module's parent into scope with use {6}, \
    and then the test can call {1}.\n\
    If you don't think private functions should be tested, there's nothing in Rust that will compel you to do so.
  ",
    "Private Function Tests".bright_magenta().bold(),
    "internal_adder".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-03-test-organization.html#listing-11-12".cyan(),
    "pub".bright_yellow().bold(),
    "tests".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html".cyan(),
    "super::*".bright_yellow().bold(),
 
  );  

  println!(
    "{0}\n\n\
    {solid_disc} The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint \
    where code is and isn't working as expected.\n\
    {solid_disc} You'll put unit tests in the {1} directory in each file with the code that they're testing.\n\
    {solid_disc} The convention is to create a module named {2} in each file to contain the test functions and to \
    annotate the module with {3}.\n\
    {solid_disc} The {4} annotation on the {2} module tells Rust to compile and run the test code only \
    when you run {5}, not when you run {6}.\n\
    {solid_disc} Integration tests go in a different directory, they don't need the {4} annotation.\n\
    {solid_disc} The attribute {7} stands for {8}.\n\
    {solid_disc} By using the {7} attribute, Cargo compiles our test code only if we actively run the tests with {5}.\n\
    {solid_disc} Tests are just Rust code, and the {2} module is just another module.\n\
    {solid_disc} Rust is not forcing you to test private functions.
    ",
    "REMEMBER".bright_white().bold(),
    "src".italic().bold(),
    "tests".bright_yellow().bold(),
    "cfg(test)".bright_yellow().bold(),
    "#[cfg(test)]".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
    "cargo build".bright_yellow().bold(),
    "cfg".bright_yellow().bold(),
    "configuration".italic().bold()
  )
}

// Header: Integration Tests. Abbreviated as it.
fn it_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Integration Tests");

  println!(
    "In Rust, integration tests are entirely external to your library.\n\
    They use your library in the same way any other code would, which means they can only call functions that are part of your \
    library's public API.\n\
    Their purpose is to test whether many parts of your library work together correctly.\n\
    Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code \
    is important as well.\n\
    To create integration tests, you first need a tests directory.
  ");

  println!(
    "{0}\n\n\
    We create a tests directory at the top level of our project directory, next to {1}.\n\
    Cargo knows to look for integration test files in this directory.\n\
    We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.\n\n\
    See: {2}, for directory structure and code sample.\n\n\
    Each file in the tests directory is a separate crate, so we need to bring our library into each test crate's scope.\n\
    For that reason, we add {3} at the top of the code, which we didn't need in the unit tests.\n\n\
    We don't need to annotate any code in {4} with {5}.\n\
    Cargo treats the tests directory specially and compiles files in this directory only when we run {6}.\n\n\
    See: {2}, for the output of {6}.\n\n\
    The three sections of output include the:\n\
    {solid_disc} unit tests,\n\
    {solid_disc} the integration test,\n\
    {solid_disc} the doc tests.\n\n\
    Note that if any test in a section fails, the following sections will not be run.\n\
    For example, if a unit test fails, there won't be any output for integration and doc tests, because those tests will only be run \
    if all unit tests are passing.\n\n\
    The first section for the unit tests is the same as we've been seeing: one line for each unit test (one named {7} that we \
    added in Listing 11-12: {8}) and then a summary line for the unit tests.\n\n\
    The integration tests section starts with the line Running {4}.\n\
    Next, there is a line for each test function in that integration test and a summary line for the results of the integration \
    test just before the {9} section starts.\n\n\
    Each integration test file has its own section, so if we add more files in the tests directory, there will be more integration \
    test sections.\n\n\
    We can still run a particular integration test function by specifying the test function's name as an argument to {6}.\n\
    To run all the tests in a particular integration test file, use the {10} argument of {6} followed by the name \
    of the file:
  ",
    "The tests Directory".bright_magenta().bold(),
    "src".italic().bold(),
    "https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory".cyan(),
    "use adder::add_two;".bright_yellow().bold(),
    "tests/integration_test.rs".italic(),
    "#[cfg(test)]".bright_yellow().bold(),
    "cargo test".bright_yellow().bold(),
    "internal".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch11-03-test-organization.html#listing-11-12".cyan(),
    "Doc-tests adder".bright_yellow().bold(),
    "--test".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    Temporarily see: {}, for the content.\n\n\
  ",
    "Submodules in Integration Tests".bright_magenta().bold(),
    "https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests".cyan()
  );

  println!(
    "{0}\n\n\
    If our project is a binary crate that only contains a {1} file and doesn't have a {2} file, we can't \
    create integration tests in the tests directory and bring functions defined in the {1} file into scope with a \
    {3} statement.\n\
    Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.\n\n\
    This is one of the reasons Rust projects that provide a binary have a straightforward {1} file that calls logic \
    that lives in the {2} file.\n\
    Using that structure, integration tests can test the library crate with {3} to make the important functionality available.\n\
    If the important functionality works, the small amount of code in the {1} file will work as well, and that small amount \
    of code doesn't need to be tested.
  ",
    "Integration Tests for Binary Crates".bright_magenta().bold(),
    "src/main.rs".italic(),
    "src/lib.rs".italic(),
    "use".bright_yellow().bold()
  );
}

// Header: Summary. Abbreviated as s.
fn s_content() {
  menu::subheader_title("Summary");

  println!(
    "Rust's testing features provide a way to specify how code should function to ensure that it continues to work as you expect, \
    even as you make changes.\n\
    Unit tests exercise different parts of a library separately and can test private implementation details.\n\
    Integration tests check that many parts of the library work together correctly, and they use the library's public API to test \
    the code in the same way external code will use it.\n\
    Even though Rust's type system and ownership rules help prevent some kinds of bugs, tests are still important to reduce logic \
    bugs having to do with how your code is expected to behave.
  ")
}
