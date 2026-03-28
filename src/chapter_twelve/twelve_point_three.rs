use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", introduction_content),
    chapter::SubHeader::new("Separating Concerns in Binary Projects", scibp_content),
    chapter::SubHeader::new("The Trade-Offs of Using clone", ttoouc_content),
    chapter::SubHeader::new("Fixing the Error Handling", fteh_content),
    chapter::SubHeader::new("Extracting Logic from main", elfm_content),
    chapter::SubHeader::new("Splitting Code into a Library Crate", scialc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction");

  println!(
    "{0}\n\n\
    Our {1} function now performs two tasks:\n\
    {solid_disc} It parses arguments\n\
    {solid_disc} and reads files.\n\n\
    As our program grows, the number of separate tasks the {1} function handles will increase.\n\
    As a function gains responsibilities, it becomes more difficult to reason about, harder to test, \
    and harder to change without breaking one of its parts.\n\
    It's best to separate functionality so that each function is responsible for one task.\n\
  ",
    "FIRST PROBLEM".bright_magenta().bold(),
    "main".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    This issue also ties into the second problem: Although {2} and {3} are configuration variables \
    to our program, variables like {4} are used to perform the program's logic.\n\
    The longer {1} becomes, the more variables we'll need to bring into scope; the more variables we \
    have in scope, the harder it will be to keep track of the purpose of each.\n\
    It's best to group the configuration variables into one structure to make their purpose clear.
  ",
    "SECOND PROBLEM".bright_magenta().bold(),
    "main".bright_yellow().bold(),
    "query".bright_yellow().bold(),
    "file_path".bright_yellow().bold(),
    "contents".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    The third problem is that we've used {1} to print an error message when reading the file fails, \
    but the error message just prints {2}.\n\
    Reading a file can fail in a number of ways:\n\
    {solid_disc} The file could be missing.\n\
    {solid_disc} We might not have permission to open it.\n\n\
    {3}
  ",
    "THIRD PROBLEM".bright_magenta().bold(),
    "expect".bright_yellow().bold(),
    "Should have been able to read the file".bright_yellow().bold(),
    "Right now, regardless of the situation, we'd print the same error message for everything, \
    which wouldn't give the user any information!".red()
  );

  println!(
    "{0}\n\n\
    Fourth, we use {1} to handle an error, and if the user runs our program without specifying enough arguments, \
    they'll get an index out of bounds error from Rust that doesn't clearly explain the problem.\n\
    It would be best if all the error-handling code were in one place so that future maintainers had only one place \
    to consult the code if the error-handling logic needed to change.\n\
    Having all the error-handling code in one place will also ensure that we're printing messages that will be meaningful \
    to our end users.
  ",
    "FOURTH PROBLEM".bright_magenta().bold(),
    "expect".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    {solid_disc} As a function gains responsibilities, it becomes more difficult to reason about, harder to test, \
    and harder to change without breaking one of its parts.\n\
    {solid_disc} It's best to separate functionality so that each function is responsible for one task.\n\
    {solid_disc} The more variables we have in scope, the harder it will be to keep track of the purpose of each. \
    It's best to group the configuration variables into one structure to make their purpose clear.\n\
    {solid_disc} It would be best if all the error-handling code were in one place so that future maintainers had only one place \
    to consult the code if the error-handling logic needed to change. \
    Having all the error-handling code in one place will also ensure that we're printing messages that will be meaningful \
    to our end users.
  ",
    "REMEMBER".bright_white().bold()
  )
}

// Header: Separating Concerns in Binary Projects. Abbreviated as scibp.
fn scibp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Separating Concerns in Binary Projects");
  
  println!(
    "The organizational problem of allocating responsibility for multiple tasks to the {0} function is common \
    to many binary projects.\n\n\
    As a result, many Rust programmers find it useful to split up the separate concerns of a binary program when \
    the {0} function starts getting large.\n\
    This process has the following steps:\n\
    {solid_disc} Split your program into a {1} file and a {2} file and move your program's logic to {2}.\n\
    {solid_disc} As long as your command line parsing logic is small, it can remain in the {0} function.\n\
    {solid_disc} When the command line parsing logic starts getting complicated, extract it from the main function \
    into other functions or types.\n\n\
    The responsibilities that remain in the {0} function after this process should be limited to the following:\n\n\
    {solid_disc} Calling the command line parsing logic with the argument values\n\
    {solid_disc} Setting up any other configuration\n\
    {solid_disc} Calling a {3} function in {2}\n\
    {solid_disc} Handling the error if {3} returns an error {2}\n\n\
    This pattern is about separating concerns:\n\
    {solid_disc} {1} handles running the program\n\
    {solid_disc}and {2} handles all the logic of the task at hand.\n\n\
    Because you can't test the {0} function directly, this structure lets you test all of your program's logic by moving \
    it out of the {0} function.\n\
    The code that remains in the main function will be small enough to verify its correctness by reading it.
  ",
    "main".bright_yellow().bold(),
    "main.rs".italic(),
    "lib.rs".italic(),
    "run".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    We'll extract the functionality for parsing arguments into a function that {1} will call.\n\n\
    See Listing 12-5: {2}, for code sample.\n\n\
    The {3} function then holds the logic that determines which argument goes in which variable and \
    passes the values back to {1}.\n\
    {1} no longer has the responsibility of determining how the command line arguments and variables correspond.\n\n\
    This rework may seem like overkill for our small program, but we're refactoring in small, incremental steps.\n\
    It's good to check your progress often, to help identify the cause of problems when they occur.
  ",
    "Extracting the Argument Parser".bright_magenta().bold(),
    "main".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-5".cyan(),
    "parse_config".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    We can take another small step to improve the {1} function further.\n\
    At the moment, we're returning a tuple, but then we immediately break that tuple into individual parts again.\n\
    This is a sign that perhaps we don't have the right abstraction yet.\n\n\
    Another indicator that shows there's room for improvement is the {2} part of {1}, which implies that the two values \
    we return are related and are both part of one configuration value.\n\
    We're not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple; \
    we'll instead put the two values into one struct and give each of the struct fields a meaningful name.\n\
    {3}.\n\n\
    See Listing 12-6:{4}, for code sample.\n\n\
    Note that we define {5} to contain owned {6} values.\n\
    The {7} variable in {8} is the owner of the argument values and is only letting the {1} function borrow them, \
    which means we'd violate Rust's borrowing rules if {5} tried to take ownership of the values in {7}.\n\n\
    There are a number of ways we could manage the {6} data; the easiest, though somewhat inefficient, route is \
    to call the {9} method on the values.\n\
    This will make a full copy of the data for the {5} instance to own, {10} than storing a reference to the string data.\n\
    However, cloning the data also makes our code very straightforward because we don't have to manage the lifetimes of the \
    references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.
  ",
    "Grouping Configuration Values".bright_magenta().bold(),
    "parse_config".bright_yellow().bold(),
    "config".bright_yellow().bold(),
    "Doing so will make it easier for future maintainers of this code to understand how the different values relate to each \
    other and what their purpose is".bright_green(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-6".cyan(),
    "Config".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "args".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "clone".bright_yellow().bold(),
    "which takes more time and memory".bright_red()
  );  

  println!(
    "{0}\n\n\
    {solid_disc} Many Rust programmers find it useful to split up the separate concerns of a binary program when \
    the {1} function starts getting large.\n\
    {solid_disc} It's best practice to move your program's logic to lib.rs.\n\
    {solid_disc} You can't test the {1} function directly, the structure of seperating concerns lets you test all of \
    your program's logic by moving it out of the {1} function.\n\
    {solid_disc} It's good to check your progress often, to help identify the cause of problems when they occur.\n\
    {solid_disc} cloning the data also makes our code very straightforward because we don't have to manage the lifetimes of the \
    references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.\n\
    {solid_disc} For now, it's okay to copy a few strings to continue making progress because you'll make these copies only once \
    and your file path and query string are very small.
  ",
    "REMEMBER".bright_white().bold(),
    "main".bright_yellow().bold()
  )

}

// Header: The Trade-Offs of Using clone. Abbreviated as ttoouc.
fn ttoouc_content() {
  menu::subheader_title("The Trade-Offs of Using clone");

  println!(
  "There's a tendency among many Rustaceans to avoid using {0} to fix ownership problems because of its runtime cost.\n\
  In Chapter 13: {1}, you'll learn how to use more efficient methods in this type of situation.\n\
  But for now, it's okay to copy a few strings to continue making progress because you'll make these copies only once \
  and your file path and query string are very small.\n\
  It's better to have a working program that's a bit inefficient than to try to hyperoptimize code on your first pass.\n\
  As you become more experienced with Rust, it'll be easier to start with the most efficient solution, but for now, \
  it's perfectly acceptable to call {0}.\n\n\
  See: {2}, for complete reading.
  ",
    "clone".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch13-00-functional-features.html".cyan(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone".cyan()
  );

  println!(
  "{0}\n\n\
  So, now that the purpose of the {1} function is to create a {2} instance, we can change {1} from a plain \
  function to a function named {2} that is associated with the {3} struct.\n\
  Making this change will make the code more idiomatic.\n\
  We can create instances of types in the standard library, such as {4}, by calling {5}.\n\
  Similarly, by changing {1} into a new function associated with {3}, we'll be able to create instances of \
  {3} by calling {6}.\n\n\
  See: {7}, for code sample.
  ",
    "Creating a Constructor for Config".bright_magenta().bold(),
    "parse_config".bright_yellow().bold(),
    "new".bright_yellow().bold(),
    "Config".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "String::new".bright_yellow().bold(),
    "Config::new".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-7".cyan()
  )
}

// Header: Fixing the Error Handling. Abbreviated as fteh.
fn fteh_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Fixing the Error Handling");

  println!(
  "Now we'll work on fixing our error handling.\n\
   Recall that attempting to access the values in the args vector at index 1 or \
  index 2 will cause the program to panic if the vector contains fewer than three items.\n\
  Try running the program without any arguments.\n\
  See: {0}, for the output
  ",
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#fixing-the-error-handling".cyan()
  );

  println!(
  "{0}\n\n\
  In Listing 12-8, we add a check in the new function that will verify that the slice is long enough before accessing index 1 \
  and index 2. If the slice isn't long enough, the program panics and displays a better error message.\n\n\
  See: {1}, for code sample.\n\n\
  See: {2}, for code output and complete reading.

  ",
    "Improving the Error Message".bright_magenta().bold(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-8".cyan(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#improving-the-error-message".cyan(),
  );

  println!(
  "{0}\n\n\
  We can instead return a {1} value that will contain a {2} instance in the successful case and will describe the problem \
  in the error case.\n\
  We're also going to change the function name from {3} to {4} because many programmers expect {3} functions to never fail.\n\
  When {5} is communicating to {6}, we can use the {1} type to signal there was a problem.\n\
  Then, we can change {6} to convert an {7} variant into a more practical error for our users without the surrounding text \
  about {8} and {9} that a call to {10} causes.\n\
  See Listing 12-9: {11}, for code sample.\n\
  See: {12}, for complete reading.
  ",
    "Returning a Result Instead of Calling panic!".bright_magenta().bold(),
    "Result".bright_yellow().bold(),
    "Config".bright_yellow().bold(),
    "new".bright_yellow().bold(),
    "build".bright_yellow().bold(),
    "Config::build".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "Err".bright_yellow().bold(),
    "thread 'main'".bright_yellow().bold(),
    "RUST_BACKTRACE".bright_yellow().bold(),
    "panic!".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-9".cyan(),
    "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#returning-a-result-instead-of-calling-panic".cyan(),
  );

  println!(
  "{0}\n\n\
  To handle the error case and print a user-friendly message, we need to update {1} to handle the {2} being returned by \
  {3}, as shown in Listing 12-10.\n\
  We'll also take the responsibility of exiting the command line tool with a nonzero error code away from {4} and \
  instead implement it by hand.\n\
  A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.\n\n\
  See Listing 12-10: {5}, for code sample.\n\n\
  In this listing, we've used a method we haven't covered in detail yet: {6}, which is defined on {7} by the \
  standard library.\n\
  Using {6} allows us to define some custom, non-panic! error handling.\n\
  If the {2} is an {8} value, this method's behavior is similar to {9}: It returns the inner value that {8} is wrapping.\n\
  However, if the value is an {10} value, this method calls the code in the closure, which is an anonymous function we define \
  and pass as an argument to {6}.\n\
  For now, you just need to know that {6} will pass the inner value of the {10}, which in this case is the static string {11} that \
  we added in Listing 12-9, to our closure in the argument {12} that appears between the vertical pipes.\n\
  The code in the closure can then use the err value when it runs.\n\n\
  We bring {13} from the standard library into scope.\n\
  The code in the closure that will be run in the error case is only two lines:\n\
  {solid_disc} We print the {12} value\n\
  {solid_disc} and then call {14}\n\n\
  The {14} function will stop the program immediately and return the number that was passed as the exit status code.\n\
  This is similar to the panic!-based handling we used in Listing 12-8, but we no longer get all the extra output.
  ",
  "Calling Config::build and Handling Errors".bright_magenta().bold(),
  "main".bright_yellow().bold(),
  "Result".bright_yellow().bold(),
  "Config::build".bright_yellow().bold(),
  "panic!".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-10".cyan(),
  "unwrap_or_else".bright_yellow().bold(),
  "Result<T, E>".bright_yellow().bold(),
  "Ok".bright_yellow().bold(),
  "unwrap".bright_yellow().bold(),
  "Err".bright_red(),
  "\"not enough arguments\"".bright_yellow().bold(),
  "err".bright_yellow().bold(),
  "process".bright_yellow().bold(),
  "process::exit".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} A nonzero exit status is a convention to signal to the process that called our program that the program exited \
  with an error state.\n\
  {solid_disc} A closure is an anonymous function.\n\
  {solid_disc} The {1} function will stop the program immediately and return the number that was passed as the exit status code.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "process::exit".bright_yellow().bold()
  )
}

// Header: Extracting Logic from main. Abbreviated as elfm.
fn elfm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Extracting Logic from main");

  println!(
  "Now that we've finished refactoring the configuration parsing, let's turn to the program's logic.\n\
  As we stated in “Separating Concerns in Binary Projects”: {0}, we'll extract a function named {3} that will hold all the \
  logic currently in the {1} function that isn't involved with setting up configuration or handling errors.\n\
  When we're done, the {1} function will be concise and easy to verify by inspection, and we'll be able to write \
  tests for all the other logic.\n\n\
  Listing 12-11 shows the small, incremental improvement of extracting a run function.\n\
  See Listing 12-11: {2}, for code sample.
  ",
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects".cyan(),
  "main".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-11".cyan(),
  "run".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  With the remaining program logic separated into the {1} function, we can improve the error handling, \
  as we did with {2} in Listing 12-9.\n\
  Instead of allowing the program to panic by calling {3}, the {1} function will return a {4} when something goes wrong.\n\
  This will let us further consolidate the logic around handling errors into {5} in a user-friendly way.\n\
  Listing 12-12 shows the changes we need to make to the signature and body of run.\n\n\
  See Listing 12-12: {6}, for code sample.\n\
  See: {7}, for complete reading.
  ",
  "Returning Errors from run".bright_magenta().bold(),
  "run".bright_yellow().bold(),
  "Config::build".bright_yellow().bold(),
  "expect".bright_yellow().bold(),
  "Result<T, E>".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-12".cyan(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#returning-errors-from-run".cyan(),
  );

  println!(
  "{0}\n\n\
  We'll check for errors and handle them using a technique similar to one we used with {1} in Listing 12-10, \
  but with a slight difference:\n\n\
  See: {2}, for code sample and complete reading.
  ",
  "Handling Errors Returned from run in main".bright_magenta().bold(),
  "Config::build".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#handling-errors-returned-from-run-in-main".cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} To use the trait object {1} type, we need to bring the {2} trait into scope.\n\
  {solid_disc} {1} matches types that implements the {2} trait. \
  This gives us flexibility to return error values that may be of different types in different error cases.\n\
  {solid_disc} The {3} keyword is short for dynamic.\n\
  {solid_disc} Using {4} in the success case: {5}, is the idiomatic way to indicate that we're calling functions for its side effects only; \
  it doesn't return a value we need.\n\
  {solid_disc} We usually use {6} if a value has two possibilities and the other case returns {4} that we don't care about.
  ",
  "REMEMBER".bright_white().bold(),
  "Box<dyn Error>".bright_yellow().bold(),
  "std::error::Error".bright_yellow().bold(),
  "dyn".bright_yellow().bold(),
  "()".bright_yellow().bold(),
  "Result<(), Box<dyn Error>>".bright_yellow().bold(),
  "if let".bright_yellow().bold(),
  
  )
}
  
// Header: Splitting Code into a Library Crate. Abbreviated as scialc.
fn scialc_content() {
  menu::subheader_title("Splitting Code into a Library Crate");

  println!(
  "Our {0} project is looking good so far! Now we'll split the {1} file and put some code into the {2} file.\n\
  That way, we can test the code and have a {1} file with fewer responsibilities.\n\
  Let's define the code responsible for searching text in {2} rather than in {1}, \
  which will let us (or anyone else using our minigrep library) call the searching function from more \
  contexts than our {0} binary.\n\n\
  First, let's define the search function signature in {2} as shown in Listing 12-13, with a body that calls the {3} macro.\n\n\
  See: Listing 12-13: {4}, for code sample.\n\n\
  We've used the pub keyword on the function definition to designate search as part of our library crate's public API.\n\
  We now have a library crate that we can use from our binary crate and that we can test!\n\n\
  See: {4}, for complete reading.
  ",
  "minigrep".bright_yellow().bold(),
  "src/main.rs".bright_yellow().bold(),
  "src/lib.rs".bright_yellow().bold(),
  "unimplemented!".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#listing-12-13".cyan()
  )
}












